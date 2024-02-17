#![allow(dead_code)]
#![allow(unused)]

use async_trait::async_trait;
use futures::Stream;
use model::{Item, Position, Tile};
use rkyv::{Archive, Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Error {
    Something, // TODO:
    WebSocket,
}

#[async_trait]
pub trait Transport: Stream<Item = Message> {
    async fn transport(&self, data: Message) -> Result<(), Error>;
}

#[derive(Debug, Archive, Deserialize, Serialize)]
pub enum Message {
    Bytes(Vec<u8>),
    ItemsCount(usize),
    Item(Item),
    Items(Vec<Item>),
    MapTilesCount(usize),
    MapTile((Position, Tile)),
    MapTiles(Vec<(Position, Tile)>),
    Loaded,
}

// FIXME: should it actually be here? probably not
impl From<tungstenite::Error> for Error {
    fn from(value: tungstenite::Error) -> Self {
        Error::WebSocket
    }
}
