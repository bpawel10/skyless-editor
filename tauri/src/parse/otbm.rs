use super::{dat, otb, Error, Parse};
use bytes::{Buf, Bytes};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::Serialize;
use std::{collections::HashMap, fs::File, io::Read, path::PathBuf};

pub struct Reader<'a> {
    pub path: &'a PathBuf,
    pub dat: &'a dat::Document,
    pub otb: &'a otb::Document,
}

impl<'a> Read for Reader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        File::open(self.path)?.read(buf)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> std::io::Result<usize> {
        File::open(self.path)?.read_to_end(buf)
    }
}

impl<'a> Reader<'a> {
    const FLOORS: u8 = 16;
    const TILE_AREA_TILES: u32 = 256 * 256;
}

impl<'a> Parse<Document> for Reader<'a> {
    fn parse(mut self) -> Result<Document, Error> {
        let mut buf = Vec::new();
        let read = self.read_to_end(&mut buf)?;
        let mut bytes = OtbmParser::new(Bytes::from(buf), self.dat, self.otb);

        let mut otbm = Document {
            map: Map {
                width: 0,
                height: 0,
                tiles: Vec::new(),
            },
        };

        bytes.get_u32_le(); // signature?

        if let Some(NodeType::Root) = bytes.get_node(false) {
            let version = bytes.get_u32_le();

            let width = bytes.get_u16_le();
            otbm.map.width = width.into();

            let height = bytes.get_u16_le();
            otbm.map.height = height.into();

            let items_major_version = bytes.get_u32_le();
            let items_minor_version = bytes.get_u32_le();

            if let Some(NodeType::MapData) = bytes.get_node(false) {
                let mut byte = bytes.get_u8();

                while byte != u8::from(SpecialCharacter::End) {
                    if let Ok(attribute) = AttributeCode::try_from(byte) {
                        match attribute {
                            AttributeCode::Description => {
                                let description = bytes.get_string();
                                println!("description {description}");
                            }
                            AttributeCode::ExtFile => {
                                let ext_file = bytes.get_string();
                                println!("ext_file {ext_file}");
                            }
                            AttributeCode::ExtSpawnFile => {
                                let ext_spawn_file = bytes.get_string();
                                println!("ext_spawn_file {ext_spawn_file}");
                            }
                            AttributeCode::ExtHouseFile => {
                                let ext_house_file = bytes.get_string();
                                println!("ext_house_file {ext_house_file}");
                            }
                            _ => (),
                        }
                    } else if byte == u8::from(SpecialCharacter::Start) {
                        let mut byte1 = byte;

                        while byte1 != u8::from(SpecialCharacter::End) {
                            let byte1_type = bytes.get_u8();

                            if byte1_type == u8::from(NodeType::TileArea) {
                                let offset_x = bytes.get_u16_le();
                                let offset_y = bytes.get_u16_le();
                                let offset_z = bytes.get_u8();

                                let mut byte2 = bytes.get_u8();

                                while byte2 != u8::from(SpecialCharacter::End) {
                                    let tile_type = bytes.get_u8();
                                    if let Ok(tile_type) = NodeType::try_from(tile_type) {
                                        if [NodeType::Tile, NodeType::HouseTile]
                                            .contains(&tile_type)
                                        {
                                            let x = bytes.get_u8();
                                            let y = bytes.get_u8();

                                            let position = Position {
                                                x: offset_x + x as u16,
                                                y: offset_y + y as u16,
                                                z: offset_z,
                                            };

                                            let mut tile = Tile {
                                                position,
                                                things: Vec::new(),
                                            };

                                            let mut ground = None;

                                            let mut byte3 = bytes.get_u8();

                                            while byte3 != u8::from(SpecialCharacter::End) {
                                                if tile.position.x == 1012
                                                    && tile.position.y == 1028
                                                {
                                                    dbg!("1012|1028 while");
                                                }

                                                if let Ok(attribute) =
                                                    AttributeCode::try_from(byte3)
                                                {
                                                    match attribute {
                                                        AttributeCode::TileFlags => {
                                                            let tile_flags = bytes.get_u32_le();
                                                        }
                                                        AttributeCode::Item => {
                                                            let ground_id = bytes.get_item_id();
                                                            // TODO: throw error when there are two different grounds on the same tile?
                                                            ground = Some(Thing {
                                                                attributes: vec![Attribute::Item(
                                                                    ground_id,
                                                                )],
                                                            })
                                                        }
                                                        _ => (),
                                                    }
                                                }

                                                if tile_type == NodeType::HouseTile {
                                                    let house_id = bytes.get_u32_le();
                                                }

                                                if let Some(thing) = bytes.get_thing(byte3, version)
                                                {
                                                    tile.things.push(thing);
                                                }

                                                byte3 = bytes.get_u8();
                                            }

                                            // TODO: throw error when there's no ground?
                                            if let Some(ground) = ground {
                                                tile.things.insert(0, ground);
                                            }

                                            otbm.map.tiles.push(tile);

                                            byte2 = bytes.get_u8();
                                        }
                                    }
                                }

                                // TODO: update progress?
                            }

                            byte1 = bytes.get_u8();
                        }
                    }

                    byte = bytes.get_u8();
                }
            }
        }

        println!("deserialized otbm");

        Ok(otbm)
    }
}

struct OtbmParser<'a> {
    bytes: Bytes,
    pos: u32,
    dat: &'a dat::Document,
    otb: &'a otb::Document,
}

impl<'a> OtbmParser<'a> {
    fn new(bytes: Bytes, dat: &'a dat::Document, otb: &'a otb::Document) -> Self {
        Self {
            bytes,
            pos: 0,
            dat,
            otb,
        }
    }

    fn get_node(&mut self, skip: bool) -> Option<NodeType> {
        let byte = self.get_u8();
        let node_byte = if skip {
            Some(byte)
        } else {
            SpecialCharacter::try_from(byte)
                .ok()
                .and_then(|char| match char {
                    SpecialCharacter::Start => Some(self.get_u8()),
                    SpecialCharacter::End => Some(char.into()),
                    _ => None,
                })
        };
        node_byte.and_then(|byte| NodeType::try_from(byte).ok())
    }

    fn get_attribute(&mut self, attribute: u8, item_id: u16) -> Option<Attribute> {
        // TODO: handle ALL attributes (returing them)
        if let Ok(attribute_code) = AttributeCode::try_from(attribute) {
            match attribute_code {
                AttributeCode::Description => None, // TODO:
                AttributeCode::ExtFile => None,     // TODO:
                AttributeCode::TileFlags => None,   // TODO:
                AttributeCode::ActionId => {
                    let action_id = self.get_u16_le();
                    None
                }
                AttributeCode::UniqueId => {
                    let unique_id = self.get_u16_le();
                    None
                }
                AttributeCode::Text => {
                    let text = self.get_string();
                    None
                }
                AttributeCode::Desc => {
                    let desc = self.get_string();
                    None
                }
                AttributeCode::Destination => {
                    let x = self.get_u16_le();
                    let y = self.get_u16_le();
                    let z = self.get_u8();
                    None
                }
                AttributeCode::Item => None, // TODO:
                AttributeCode::DepotId => {
                    let depot_id = self.get_u16_le();
                    None
                }
                AttributeCode::ExtSpawnFile => None, // TODO:
                AttributeCode::RuneCharges => {
                    let rune_charges = self.get_u8();
                    None
                }
                AttributeCode::ExtHouseFile => None, // TODO:
                AttributeCode::HouseDoorId => {
                    let house_door_id = self.get_u8();
                    None
                }
                AttributeCode::Count => {
                    let value = self.get_u8();
                    let dat_item = self.dat.items.get(&item_id);
                    if let Some(dat_item) = dat_item {
                        if dat_item.stackable {
                            Some(Attribute::Count(value))
                        } else if dat_item.splash || dat_item.fluid_container {
                            Some(Attribute::Fluid(value))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                AttributeCode::Duration => None,      // TODO:
                AttributeCode::DecayingState => None, // TODO:
                AttributeCode::WrittenDate => None,   // TODO:
                AttributeCode::WrittenBy => None,     // TODO:
                AttributeCode::SleeperGuid => None,   // TODO:
                AttributeCode::SleepStart => None,    // TODO:
                AttributeCode::Charges => {
                    let charges = self.get_u16_le();
                    None
                }
                AttributeCode::Map => None, // TODO:
            }
        } else {
            None
        }
    }

    fn get_u8(&mut self) -> u8 {
        let mut u8 = self.bytes.get_u8();
        self.pos += 1;
        if u8 == u8::from(SpecialCharacter::Escape) {
            u8 = self.bytes.get_u8();
            self.pos += 1;
        }
        u8
    }

    fn get_u16_le(&mut self) -> u16 {
        u16::from_le_bytes([0; 2].map(|_| self.get_u8()))
    }

    fn get_u32_le(&mut self) -> u32 {
        u32::from_le_bytes([0; 4].map(|_| self.get_u8()))
    }

    fn get_string(&mut self) -> String {
        let length = self.get_u16_le();
        let mut chars = Vec::new();
        for _ in 0..length {
            chars.push(self.get_u8());
        }
        String::from_utf8_lossy(&chars).to_string()
    }

    fn get_thing(&mut self, byte: u8, version: u32) -> Option<Thing> {
        if byte == u8::from(SpecialCharacter::Start) && self.get_u8() == u8::from(NodeType::Item) {
            let mut item_id = self.get_item_id();
            let mut count = None;
            let mut fluid = None;
            let mut children = Vec::new();
            let mut attributes = Vec::new();

            let mut byte2 = self.get_u8();

            while byte2 != u8::from(SpecialCharacter::End) {
                if byte2 == u8::from(SpecialCharacter::Start) {
                    let child = self.get_thing(byte2, version);
                    if let Some(child) = child {
                        children.push(child);
                    }
                } else if byte2 == u8::from(AttributeCode::Item) {
                    item_id = self.get_item_id();
                    if version == 1 {
                        let dat_item = self.dat.items.get(&item_id);
                        if let Some(dat_item) = dat_item {
                            if dat_item.stackable {
                                count = Some(self.get_u8());
                            }
                            if dat_item.splash || dat_item.fluid_container {
                                fluid = Some(self.get_u8());
                            }
                        }
                    }
                } else {
                    let attr = self.get_attribute(byte2, item_id);
                    if let Some(attr) = attr {
                        attributes.push(attr);
                    }
                }

                byte2 = self.get_u8();
            }

            attributes.push(Attribute::Item(item_id));
            if let Some(count) = count {
                attributes.push(Attribute::Count(count));
            }
            if let Some(fluid) = fluid {
                attributes.push(Attribute::Fluid(fluid));
            }
            if !children.is_empty() {
                attributes.push(Attribute::Container(children));
            }

            Some(Thing { attributes })
        } else {
            None
        }
    }

    fn get_item_id(&mut self) -> u16 {
        let server_id = self.get_u16_le();
        let otb_item = self.otb.items.get(&server_id).unwrap();
        otb_item.client_id
    }
}

#[repr(u8)]
#[derive(IntoPrimitive, TryFromPrimitive)]
enum SpecialCharacter {
    Start = 0xFE,
    End = 0xFF,
    Escape = 0xFD,
}

#[repr(u8)]
#[derive(IntoPrimitive, PartialEq, TryFromPrimitive)]
enum NodeType {
    Root = 0x00,
    RootV1 = 0x01,
    MapData = 0x02,
    ItemDef = 0x03,
    TileArea = 0x04,
    Tile = 0x05,
    Item = 0x06,
    TileSquare = 0x07,
    TileRef = 0x08,
    Spawns = 0x09,
    SpawnArea = 0x0A,
    Monster = 0x0B,
    Towns = 0x0C,
    Town = 0x0D,
    HouseTile = 0x0E,
    Waypoints = 0x0F,
    Waypoint = 0x10,
}

#[repr(u8)]
#[derive(IntoPrimitive, TryFromPrimitive)]
enum AttributeCode {
    Description = 0x01,
    ExtFile = 0x02,
    TileFlags = 0x03,
    ActionId = 0x04,
    UniqueId = 0x05,
    Text = 0x06,
    Desc = 0x07, // what is it?
    Destination = 0x08,
    Item = 0x09,
    DepotId = 0x0A,
    ExtSpawnFile = 0x0B,
    RuneCharges = 0x0C,
    ExtHouseFile = 0x0D,
    HouseDoorId = 0x0E,
    Count = 0x0F,
    Duration = 0x10,
    DecayingState = 0x11,
    WrittenDate = 0x12,
    WrittenBy = 0x13,
    SleeperGuid = 0x14,
    SleepStart = 0x15,
    Charges = 0x16,
    Map = 0x80,
}

#[derive(Debug, Serialize)]
pub enum Attribute {
    Item(u16),
    Count(u8),
    Fluid(u8),
    Container(Vec<Thing>),
}

#[derive(Debug, Serialize)]
pub struct Thing {
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize)]
pub struct Position {
    pub x: u16,
    pub y: u16,
    pub z: u8,
}

#[derive(Debug, Serialize)]
pub struct Tile {
    pub position: Position,
    pub things: Vec<Thing>,
}

#[derive(Serialize)]
pub struct Map {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Tile>,
}

#[derive(Serialize)]
pub struct Document {
    pub map: Map,
}
