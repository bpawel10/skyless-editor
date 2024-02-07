use crate::Entity;
use rkyv::{Archive, Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Archive, Deserialize, Serialize)]
#[archive(bound(serialize = "__S: rkyv::ser::ScratchSpace + rkyv::ser::Serializer"))]
pub struct Container(#[omit_bounds] pub Vec<Entity>);
