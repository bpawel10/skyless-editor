use super::Message;
use futures::Stream;
use js_sys::{ArrayBuffer, Uint8Array};
use tokio::sync::mpsc::{channel, Receiver};
use wasm_bindgen::{prelude::Closure, JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::{BinaryType, MessageEvent};

pub struct WebSocket {
    client: web_sys::WebSocket,
    receiver: Receiver<Message>,
}

impl WebSocket {
    pub fn new(ws_url: &str) -> Self {
        let client = web_sys::WebSocket::new(&format!("ws://{}", ws_url)).unwrap();
        client.set_binary_type(BinaryType::Arraybuffer); // TODO: check which one is faster, arraybuffer or blob
        let (sender, receiver) = channel::<Message>(1024);
        let onmessage_callback = Closure::wrap(Box::new(move |event: MessageEvent| {
            let sender = sender.clone();
            spawn_local(async move {
                if let Ok(array_buffer) = event.data().dyn_into::<ArrayBuffer>() {
                    let array_buffer = Uint8Array::new(&array_buffer).to_vec();
                    let msg =
                        unsafe { rkyv::from_bytes_unchecked::<Message>(&array_buffer).unwrap() };
                    sender.send(msg).await.unwrap(); // FIXME: unwraps
                }
            });
        }) as Box<dyn FnMut(MessageEvent)>);
        client.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();
        Self { client, receiver }
    }
}

impl Stream for WebSocket {
    type Item = Message;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.receiver.poll_recv(cx)
    }
}
