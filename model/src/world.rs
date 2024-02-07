use super::{Area, Entity, Position, Tile};
use rkyv::{Archive, Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, Default, Archive, Deserialize, Serialize)]
pub struct World {
    width: u32,
    height: u32,
    areas: Vec<Area>,
    tiles: HashMap<Position, Tile>,
}

impl World {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            areas: vec![Area::root()],
            tiles: HashMap::new(),
        }
    }

    pub fn empty() -> Self {
        World::new(0, 0)
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn root_area_mut(&mut self) -> &mut Area {
        self.areas.iter_mut().find(|area| area.is_root()).unwrap()
    }

    pub fn add_tile(&mut self, position: Position, tile: Tile) {
        self.tiles.insert(position.clone(), tile);
        self.root_area_mut().add_tile(position);
    }

    pub fn add_entity(&mut self, position: Position, entity: Entity, index: Option<u8>) {
        let tile = self.tiles.get_mut(&position);
        match tile {
            Some(tile) => tile.entities.insert(
                index
                    .map(|index| index.into())
                    .unwrap_or(tile.entities.len()),
                entity,
            ),
            None => {
                self.tiles.insert(
                    position.clone(),
                    Tile {
                        attributes: HashMap::new(),
                        entities: vec![entity],
                    },
                );
                let root_area = self.areas.iter_mut().find(|area| area.is_root()).unwrap();
                root_area.add_tile(position);
            }
        }
    }

    pub fn tiles(&self) -> &HashMap<Position, Tile> {
        &self.tiles
    }

    pub fn into_tiles(self) -> HashMap<Position, Tile> {
        self.tiles
    }
}
