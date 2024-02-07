use super::Position;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Clone, Archive, Deserialize, Serialize)]
pub struct Area {
    name: String,
    parent: Option<String>,
    tiles: Vec<Position>,
}

impl Area {
    const ROOT: &'static str = "root";

    pub fn new(name: String, parent: Option<String>, tiles: Option<Vec<Position>>) -> Self {
        Self {
            name,
            parent,
            tiles: tiles.unwrap_or_default(),
        }
    }

    pub fn root() -> Self {
        Area::new(Area::ROOT.to_string(), None, None)
    }

    pub fn is_root(&self) -> bool {
        self.name == Area::ROOT
    }

    pub fn add_tile(&mut self, position: Position) {
        self.tiles.push(position);
    }
}
