use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, Default, Archive, Deserialize, Serialize)]
pub struct Offset {
    pub x: u16,
    pub y: u16,
}
