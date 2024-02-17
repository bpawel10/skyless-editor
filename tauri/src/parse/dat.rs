use super::{Error, Parse};
use bytes::{Buf, Bytes};
use model::Offset;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::{collections::HashMap, io::Read};

pub struct Document {
    pub signature: u32,
    pub items: HashMap<u16, Item>,
}

pub struct Item {
    pub id: u16,
    pub minimap_color: Option<u16>,
    pub ground: bool,
    pub stackable: bool,
    pub splash: bool,
    pub fluid_container: bool,
    pub draw_offset: Offset,
    pub height_offset: Offset,
    pub textures: Textures,
}

#[derive(Debug)]
pub struct Textures {
    pub width: u8,
    pub height: u8,
    pub layers: u8,
    pub patterns_x: u8,
    pub patterns_y: u8,
    pub patterns_z: u8,
    pub frames: u8,
    pub sprites: Vec<u16>,
}

#[repr(u8)]
#[derive(IntoPrimitive)]
enum SpecialCharacter {
    FlagsEnd = 0xFF,
}

#[repr(u8)]
#[derive(TryFromPrimitive)]
enum ItemFlag {
    Ground = 0x00,
    OnTop = 0x01,
    WalkThroughDoors = 0x02,
    WalkThroughArches = 0x03,
    Container = 0x04,
    Stackable = 0x05,
    Ladder = 0x06,
    Usable = 0x07,
    Rune = 0x08,
    Writeable = 0x09,
    Readable = 0x0A,
    FluidContainer = 0x0B,
    Splash = 0x0C,
    Blocking = 0x0D,
    Immoveable = 0x0E,
    BlocksMissile = 0x0F,
    BlocksMonsterMovement = 0x10,
    Equipable = 0x11,
    Hangable = 0x12,
    Horizontal = 0x13,
    Vertical = 0x14,
    Rotateable = 0x15,
    LightInfo = 0x16,
    Unknown1 = 0x17,
    FloorChangeDown = 0x18,
    DrawOffset = 0x19,
    Height = 0x1A,
    DrawWithHeightOffsetForAllParts = 0x1B,
    LifeBarOffset = 0x1C,
    MinimapColor = 0x1D,
    FloorChange = 0x1E,
    Unknown2 = 0x1F,
}

impl<T: Read + Sized> Parse<Document> for T {
    fn parse(mut self) -> Result<Document, Error> {
        let mut buf = Vec::new();
        self.read_to_end(&mut buf)?;
        let mut bytes = Bytes::from(buf);

        let signature = bytes.get_u32_le();

        let items_count = bytes.get_u16_le();
        let outfits_count = bytes.get_u16_le();
        let effects_count = bytes.get_u16_le();
        let distance_effects_count = bytes.get_u16_le();

        // TODO: handle outfits, effects and distances

        let mut items = HashMap::new();

        for i in 0..items_count {
            let mut ground = false;
            let mut stackable = false;
            let mut splash = false;
            let mut fluid_container = false;
            let mut draw_offset = Offset::default();
            let mut height_offset = Offset::default();
            let mut minimap_color = None;

            let mut byte = bytes.get_u8();
            while byte != u8::from(SpecialCharacter::FlagsEnd) {
                if let Ok(flag) = ItemFlag::try_from(byte) {
                    match flag {
                        ItemFlag::Stackable => stackable = true,
                        ItemFlag::Splash => splash = true,
                        ItemFlag::FluidContainer => fluid_container = true,
                        ItemFlag::MinimapColor => minimap_color = Some(bytes.get_u16_le()),
                        ItemFlag::Height => {
                            let height = bytes.get_u16_le();
                            height_offset.x = height;
                            height_offset.y = height;
                        }
                        ItemFlag::Ground => {
                            // TODO: handle speed?
                            let speed = bytes.get_u16_le();

                            ground = true;
                        }
                        ItemFlag::Writeable | ItemFlag::Readable | ItemFlag::FloorChange => {
                            // TODO: handle these properties
                            bytes.get_u16_le();
                        }
                        ItemFlag::LightInfo => {
                            // TODO: handle this property
                            bytes.get_u32_le();
                        }
                        ItemFlag::DrawOffset => {
                            draw_offset.x = bytes.get_u16_le();
                            draw_offset.y = bytes.get_u16_le();
                        }
                        _ => (),
                    }
                }
                byte = bytes.get_u8();
            }

            let width = bytes.get_u8();
            let height = bytes.get_u8();
            if width > 1 || height > 1 {
                bytes.get_u8(); // TODO: check in rme/otclient code maybe we can find what this byte contains
            }
            let layers = bytes.get_u8();
            let patterns_x = bytes.get_u8();
            let patterns_y = bytes.get_u8();
            let patterns_z = bytes.get_u8();
            let frames = bytes.get_u8();

            let sprites_count = width as u16
                * height as u16
                * layers as u16
                * patterns_x as u16
                * patterns_y as u16
                * patterns_z as u16
                * frames as u16;

            let mut sprites = Vec::new();

            for _ in 0..sprites_count {
                sprites.push(bytes.get_u16_le());
            }

            let textures = Textures {
                width,
                height,
                layers,
                patterns_x,
                patterns_y,
                patterns_z,
                frames,
                sprites,
            };

            let id = i + 100;
            let item = Item {
                id,
                minimap_color,
                ground,
                stackable,
                splash,
                fluid_container,
                draw_offset,
                height_offset,
                textures,
            };
            items.insert(id, item);

            // TODO: update progress?
        }

        Ok(Document { signature, items })
    }
}
