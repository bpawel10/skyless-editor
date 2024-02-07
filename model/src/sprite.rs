use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Sprite {
    id: u16,
    pixels: Vec<u8>,
}

impl Sprite {
    pub const SIZE: u16 = 32;
    pub const BYTES: u16 = Self::SIZE * Self::SIZE * 4;

    pub fn new(id: u16, pixels: Vec<u8>) -> Self {
        Self { id, pixels }
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }
}
