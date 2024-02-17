use super::{Error, Parse};
use bytes::{Buf, Bytes};
use num_enum::TryFromPrimitive;
use std::{
    collections::HashMap,
    io::Read,
    mem::{size_of_val, take},
};

pub struct Document {
    pub major_version: u32,
    pub minor_version: u32,
    pub build_version: u32,
    pub items: HashMap<u16, Item>,
}

#[derive(Debug)]
pub struct Item {
    pub group: ItemGroup,
    pub flags: u32, // TODO: decode it into a vector of flags
    pub server_id: u16,
    pub client_id: u16,
    pub name: Option<String>,
    pub ground_speed: Option<u16>,
    pub sprite_hash: Option<Vec<u8>>,
    pub minimap_color: Option<u16>,
    pub max_read_write_chars: Option<u16>,
    pub max_read_chars: Option<u16>,
    pub light: Option<Light>,
    pub stack_order: Option<StackOrder>,
    pub trade_as: Option<u16>,
}

#[derive(Debug)]
pub struct Light {
    pub level: u16,
    pub color: u16,
}

#[derive(Debug)]
struct Node {
    pub node: Option<Bytes>,
    children: Vec<Node>,
}

#[repr(u8)]
#[derive(TryFromPrimitive)]
enum SpecialCharacter {
    Start = 0xFE,
    End = 0xFF,
    Escape = 0xFD,
}

#[repr(u8)]
#[derive(TryFromPrimitive, Debug)]
pub enum ItemGroup {
    None = 0,
    Ground = 1,
    Container = 2,
    Weapon = 3,
    Ammunition = 4,
    Armor = 5,
    Changes = 6,
    Teleport = 7,
    MagicField = 8,
    Writable = 9,
    Key = 10,
    Splash = 11,
    Fluid = 12,
    Door = 13,
    Deprecated = 14,
}

#[repr(u8)]
#[derive(TryFromPrimitive, Debug)]
pub enum ItemType {
    None = 0,
    Ground = 1,
    Container = 2,
    Fluid = 3,
    Splash = 4,
    Deprecated = 5,
}

#[repr(u8)]
#[derive(TryFromPrimitive, Debug)]
pub enum StackOrder {
    None = 0,
    Border = 1,
    Bottom = 2,
    Top = 3,
}

enum ItemFlag {
    None = 0,
    Unpassable = 1 << 0,
    BlockMissiles = 1 << 1,
    BlockPathfinder = 1 << 2,
    HasElevation = 1 << 3,
    MultiUse = 1 << 4,
    Pickupable = 1 << 5,
    Movable = 1 << 6,
    Stackable = 1 << 7,
    FloorChangeDown = 1 << 8,
    FloorChangeNorth = 1 << 9,
    FloorChangeEast = 1 << 10,
    FloorChangeSouth = 1 << 11,
    FloorChangeWest = 1 << 12,
    StackOrder = 1 << 13,
    Readable = 1 << 14,
    Rotatable = 1 << 15,
    Hangable = 1 << 16,
    HookSouth = 1 << 17,
    HookEast = 1 << 18,
    CanNotDecay = 1 << 19,
    AllowDistanceRead = 1 << 20,
    Unused = 1 << 21,
    ClientCharges = 1 << 22,
    IgnoreLook = 1 << 23,
    IsAnimation = 1 << 24,
    FullGround = 1 << 25,
    ForceUse = 1 << 26,
}

#[repr(u8)]
#[derive(TryFromPrimitive)]
enum ItemAttribute {
    ServerId = 0x10,
    ClientId = 0x11,
    Name = 0x12,
    GroundSpeed = 0x14,
    SpriteHash = 0x20,
    MinimapColor = 0x21,
    MaxReadWriteChars = 0x22,
    MaxReadChars = 0x23,
    Light = 0x2A,
    StackOrder = 0x2B,
    TradeAs = 0x2D,
}

impl<T: Read + Sized> Parse<Document> for T {
    fn parse(mut self) -> Result<Document, Error> {
        let mut buf = Vec::new();
        self.read_to_end(&mut buf)?;
        let bytes = Bytes::from(buf);
        let mut otb = Node::new(bytes.slice(4..bytes.len()));

        otb.get_u8();
        otb.get_u32_le();

        let mut major_version = None;
        let mut minor_version = None;
        let mut build_version = None;

        let attr = otb.get_u8().unwrap();

        // TODO: use enum instead
        if attr == 0x01 {
            let data_length = otb.get_u16_le().unwrap();
            major_version = otb.get_u32_le();
            minor_version = otb.get_u32_le();
            build_version = otb.get_u32_le();
            otb.get_slice(
                usize::from(data_length)
                    - size_of_val(&major_version)
                    - size_of_val(&minor_version)
                    - size_of_val(&build_version),
            );
        }

        let mut item_nodes = otb.children;
        let mut items = HashMap::new();

        item_nodes.iter_mut().for_each(|node| {
            let group = node.get_u8().unwrap();
            let flags = node.get_u32_le().unwrap();

            let mut server_id = None;
            let mut client_id = None;
            let mut name = None;
            let mut ground_speed = None;
            let mut sprite_hash = None;
            let mut minimap_color = None;
            let mut max_read_write_chars = None;
            let mut max_read_chars = None;
            let mut light = None;
            let mut stack_order = None;
            let mut trade_as = None;

            while node.has_remaining() {
                let attr = node.get_u8().unwrap();
                let data_length = node.get_u16_le().unwrap();

                if let Ok(attr) = ItemAttribute::try_from(attr) {
                    match attr {
                        ItemAttribute::ServerId => server_id = node.get_u16_le(),
                        ItemAttribute::ClientId => client_id = node.get_u16_le(),
                        ItemAttribute::Name => {
                            let length = node.get_u16_le().unwrap();
                            name = Some(
                                String::from_utf8_lossy(&node.get_slice(length.into()).unwrap())
                                    .to_string(),
                            );
                        }
                        ItemAttribute::GroundSpeed => ground_speed = node.get_u16_le(),
                        ItemAttribute::SpriteHash => {
                            sprite_hash = node.get_slice(data_length.into())
                        }
                        ItemAttribute::MinimapColor => minimap_color = node.get_u16_le(),
                        ItemAttribute::MaxReadWriteChars => {
                            max_read_write_chars = node.get_u16_le()
                        }
                        ItemAttribute::MaxReadChars => max_read_chars = node.get_u16_le(),
                        ItemAttribute::Light => {
                            let level = node.get_u16_le().unwrap();
                            let color = node.get_u16_le().unwrap();
                            light = Some(Light { level, color });
                        }
                        ItemAttribute::StackOrder => {
                            stack_order = node
                                .get_u8()
                                .and_then(|stack_order| StackOrder::try_from(stack_order).ok())
                        }
                        ItemAttribute::TradeAs => trade_as = node.get_u16_le(),
                        _ => {
                            node.get_slice(data_length.into());
                        }
                    }
                }
            }

            let item = Item {
                group: ItemGroup::try_from(group).unwrap(),
                flags,
                server_id: server_id.unwrap(),
                client_id: client_id.unwrap(),
                name,
                ground_speed,
                sprite_hash,
                minimap_color,
                max_read_write_chars,
                max_read_chars,
                light,
                stack_order,
                trade_as,
            };
            items.insert(item.server_id, item);

            // TODO: update progress?
        });

        Ok(Document {
            major_version: major_version.unwrap(),
            minor_version: minor_version.unwrap(),
            build_version: build_version.unwrap(),
            items,
        })
    }
}

impl Node {
    fn new(bytes: Bytes) -> Self {
        let mut otb_node = Node {
            node: None,
            children: Vec::new(),
        };

        let mut node = Vec::new();
        let mut node_escaped = Vec::new();
        let mut escape = false;

        bytes.into_iter().for_each(|byte| {
            if escape {
                node.push(byte);
                node_escaped.push(byte);
                escape = false;
                return;
            }

            match SpecialCharacter::try_from(byte) {
                Ok(SpecialCharacter::Start) => {
                    if !node.is_empty() {
                        if otb_node.node.is_some() {
                            let child = Node::new(Bytes::from(take(&mut node)));
                            otb_node.children.push(child);
                        } else {
                            otb_node.set_node(take(&mut node_escaped));
                        }
                    }
                    node.clear();
                    node_escaped.clear();
                }
                Ok(SpecialCharacter::Escape) => {
                    node.push(byte);
                    escape = true;
                }
                Ok(SpecialCharacter::End) => (),
                Err(_) => {
                    node.push(byte);
                    node_escaped.push(byte);
                }
            }
        });

        if otb_node.node.is_none() {
            otb_node.set_node(node_escaped);
        }

        otb_node
    }

    fn has_remaining(&self) -> bool {
        match self.node.as_ref() {
            Some(node) => node.has_remaining(),
            None => false,
        }
    }

    fn get_u8(&mut self) -> Option<u8> {
        self.node.as_mut().map(|node| node.get_u8())
    }

    fn get_u16_le(&mut self) -> Option<u16> {
        self.node.as_mut().map(|node| node.get_u16_le())
    }

    fn get_u32_le(&mut self) -> Option<u32> {
        self.node.as_mut().map(|node| node.get_u32_le())
    }

    fn get_slice(&mut self, length: usize) -> Option<Vec<u8>> {
        self.node.as_mut().and_then(|node| {
            if node.remaining() >= length {
                let mut slice = vec![0; length];
                node.copy_to_slice(&mut slice);
                Some(slice)
            } else {
                None
            }
        })
    }

    fn set_node(&mut self, node: Vec<u8>) {
        self.node = Some(Bytes::from(node));
    }
}
