use rkyv::{Archive, Deserialize, Serialize};
use std::{cmp::Ordering, fmt::Display};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Archive, Deserialize, Serialize, PartialOrd, Ord)]
#[archive_attr(derive(PartialEq, Eq, Hash))]
pub struct Position {
    pub x: u16,
    pub y: u16,
    pub z: u8,
    pub stack_pos: Option<u16>,
}

impl Position {
    pub fn x(mut self, x: u16) -> Self {
        self.x = x;
        self
    }

    pub fn y(mut self, y: u16) -> Self {
        self.y = y;
        self
    }

    pub fn z(mut self, z: u8) -> Self {
        self.z = z;
        self
    }

    pub fn stack_pos(mut self, stack_pos: Option<u16>) -> Self {
        self.stack_pos = stack_pos;
        self
    }

    pub fn diff(&self, second: Position) -> (i32, i32, i32) {
        (
            (second.x as i32) - (self.x as i32),
            (second.y as i32) - (self.y as i32),
            (second.z as i32) - (self.z as i32),
        )
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("({},{},{})", self.x, self.y, self.z))
    }
}

// TODO: implement pos! macro instead?
#[allow(non_snake_case)]
pub fn Position(x: u16, y: u16, z: u8) -> Position {
    Position {
        x,
        y,
        z,
        stack_pos: None,
    }
}
