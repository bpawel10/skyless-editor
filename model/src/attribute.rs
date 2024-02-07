use super::attributes::{Container, Count, Fluid, Item};
use rkyv::{Archive, Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};
use strum::Display;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Display, Archive, Deserialize, Serialize)]
#[strum(serialize_all = "snake_case")]
pub enum Attribute {
    Container(Container),
    Count(Count),
    Fluid(Fluid),
    Item(Item),
}

pub type AttributeType = Attribute;
pub type AttributesType = HashMap<String, AttributeType>;
