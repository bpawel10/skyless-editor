use super::{attribute::AttributesType, entity::Entity};
use rkyv::{Archive, Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Default, Clone, Archive, Deserialize, Serialize)]
pub struct Tile {
    pub attributes: AttributesType,
    pub entities: Vec<Entity>,
}
