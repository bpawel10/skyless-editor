use crate::transport::{Error, Message as TransportMessage, Transport};
use async_trait::async_trait;
use futures::{lock::Mutex, sink::SinkExt, stream::SplitSink, Stream, StreamExt};
use model::{Item, Position, Tile};
use rkyv::{Archive, Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::Arc,
};
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::WebSocketStream;
use tungstenite::Message;

#[derive(Archive, Deserialize, Serialize)]
pub enum WebSocketMessage {
    ItemsCount(usize),
    Item(Item),
    Items(HashMap<u16, Item>),
    MapTilesCount(usize),
    MapTiles(HashMap<Position, Tile>),
    Bytes(Vec<u8>),
}

pub struct WebSocket {
    url: SocketAddr,
    client: Arc<Mutex<Option<SplitSink<WebSocketStream<TcpStream>, Message>>>>,
}

impl WebSocket {
    pub fn init<R: Runtime>() -> TauriPlugin<R> {
        Builder::new("websocket")
            .setup(|app| {
                let app = app.clone();
                tauri::async_runtime::spawn(async move {
                    let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0))
                        .await
                        .unwrap();

                    app.manage(Arc::new(WebSocket {
                        url: listener.local_addr().unwrap(),
                        client: Arc::new(Mutex::new(None)),
                    }));

                    while let Ok((client, _)) = listener.accept().await {
                        let ws_stream = tokio_tungstenite::accept_async(client).await.unwrap();
                        let (write, _) = ws_stream.split();
                        let ws = app.state::<Arc<WebSocket>>();
                        let mut client = ws.client.lock().await;
                        *client = Some(write);
                    }
                });
                Ok(())
            })
            .build()
    }

    pub fn url(&self) -> SocketAddr {
        self.url
    }
}

#[async_trait]
impl Transport for WebSocket {
    async fn transport(&self, data: TransportMessage) -> Result<(), Error> {
        let mut client = self.client.lock().await;
        let mut sink = client.as_mut().ok_or(Error::Something)?; // FIXME: return proper error

        sink.send(Message::Binary(
            // FIXME: 1024 is a kinda randomly chose number of bytes to allocate, try to find a better one
            rkyv::to_bytes::<TransportMessage, 1024>(&data)
                .unwrap()
                .to_vec(),
        ))
        .await
        .unwrap(); // FIXME: unwraps
        Ok(())
    }
}

impl Stream for WebSocket {
    type Item = TransportMessage;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        todo!()
    }
}
