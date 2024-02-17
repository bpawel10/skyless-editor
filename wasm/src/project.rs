use crate::{progress::Progress, transport::WebSocket};
use futures::{Stream, StreamExt};
use js_sys::Function;
use model::{Item, World};
use std::collections::HashMap;
use transport::Message;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub struct Project {
    #[wasm_bindgen(skip)]
    pub data: ProjectData,
    transport: Box<dyn Stream<Item = Message>>,
}

#[wasm_bindgen]
pub struct ProjectData {
    #[wasm_bindgen(skip)]
    pub assets: ProjectAssets,
    #[wasm_bindgen(skip)]
    pub world: World,
}

#[wasm_bindgen]
pub struct ProjectAssets {
    #[wasm_bindgen(skip)]
    pub items: HashMap<u16, Item>,
}

#[wasm_bindgen]
impl Project {
    pub async fn load(ws_url: &str, callback: Function) -> Self {
        let mut ws = WebSocket::new(ws_url);

        let mut items = HashMap::new();
        let mut world = World::empty();

        let mut total_items = 0;
        let mut total_tiles = 0;

        while let Some(msg) = ws.next().await {
            match msg {
                Message::ItemsCount(total) => {
                    total_items = total;
                }
                Message::Item(item) => {
                    items.insert(item.id, item);

                    if total_items > 0
                        && items.len() == total_items
                        && total_tiles > 0
                        && world.tiles().len() == total_tiles
                    {
                        break;
                    }

                    callback
                        .call1(
                            &JsValue::null(),
                            &serde_wasm_bindgen::to_value(&Progress {
                                progress: items.len() as f32 / total_items as f32,
                                label: Some("Loading items".to_string()),
                            })
                            .unwrap(),
                        )
                        .unwrap();
                }
                Message::Items(msg_items) => {
                    for item in msg_items.into_iter() {
                        items.insert(item.id, item);
                    }

                    if total_items > 0
                        && items.len() == total_items
                        && total_tiles > 0
                        && world.tiles().len() == total_tiles
                    {
                        break;
                    }

                    callback
                        .call1(
                            &JsValue::null(),
                            &serde_wasm_bindgen::to_value(&Progress {
                                progress: items.len() as f32 / total_items as f32,
                                label: Some("Loading items".to_string()),
                            })
                            .unwrap(),
                        )
                        .unwrap();
                }
                Message::MapTilesCount(count) => {
                    total_tiles = count;
                }
                Message::MapTile((position, tile)) => {
                    world.add_tile(position, tile);

                    if total_items > 0
                        && items.len() == total_items
                        && total_tiles > 0
                        && world.tiles().len() == total_tiles
                    {
                        break;
                    }

                    callback
                        .call1(
                            &JsValue::null(),
                            &serde_wasm_bindgen::to_value(&Progress {
                                progress: world.tiles().len() as f32 / total_tiles as f32,
                                label: Some("Loading world".to_string()),
                            })
                            .unwrap(),
                        )
                        .unwrap();
                }
                Message::MapTiles(map_tiles) => {
                    for (position, tile) in map_tiles.into_iter() {
                        world.add_tile(position, tile);
                    }

                    if total_items > 0
                        && items.len() == total_items
                        && total_tiles > 0
                        && world.tiles().len() == total_tiles
                    {
                        break;
                    }

                    if items.len() == total_items {
                        callback
                            .call1(
                                &JsValue::null(),
                                &serde_wasm_bindgen::to_value(&Progress {
                                    progress: world.tiles().len() as f32 / total_tiles as f32,
                                    label: Some("Loading world".to_string()),
                                })
                                .unwrap(),
                            )
                            .unwrap();
                    }
                }
                _ => (), // TODO: handle the rest of messages
            };
        }

        Self {
            data: ProjectData {
                assets: ProjectAssets { items },
                world,
            },
            transport: Box::new(ws),
        }
    }

    #[wasm_bindgen(getter, js_name = tilesLen)]
    pub fn tiles_len(&self) -> usize {
        self.data.world.tiles().len()
    }
}
