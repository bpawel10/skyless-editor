use super::Parse;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashMap;
use std::io::Read;

#[derive(Debug)]
pub enum Error {
    Malformed,
}

pub const SPRITE_SIZE: usize = 32;
const SPRITE_PIXELS: usize = SPRITE_SIZE * SPRITE_SIZE;
const BYTES_PER_PIXEL: usize = 4;
const SPRITE_BYTES: usize = SPRITE_PIXELS * BYTES_PER_PIXEL;

#[derive(Debug, Clone)]
pub struct SpriteBytes([u8; SPRITE_BYTES]);

impl TryFrom<Vec<u8>> for SpriteBytes {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        value
            .try_into()
            .map(SpriteBytes)
            .map_err(|_| Error::Malformed)
    }
}

impl From<Error> for super::Error {
    fn from(_: Error) -> Self {
        super::Error::Malformed
    }
}

impl From<SpriteBytes> for Vec<u8> {
    fn from(value: SpriteBytes) -> Self {
        value.0.to_vec()
    }
}

pub struct Document {
    pub signature: u32,
    pub count: u16,
    pub sprites: HashMap<u16, SpriteBytes>,
}

// FIXME: this parser can panic because of bytes::Bytes methods
// to avoid that we can use byteorder crate instead
// or add some checks here before we try to read bytes

impl<T: Read + Sized> Parse<Document> for T {
    fn parse(mut self) -> Result<Document, super::Error> {
        let mut buf = Vec::new();
        self.read_to_end(&mut buf)?;
        let mut bytes = Bytes::from(buf);
        let mut bytes_clone = bytes.clone();

        let signature = bytes.get_u32_le();
        let count = bytes.get_u16_le();

        let offsets = (0..count)
            .map(|index| (bytes.get_u32_le(), index + 1))
            .collect::<Vec<(u32, u16)>>();

        let sprites = offsets
            .par_iter()
            .filter(|(offset, _)| *offset != 0)
            .map(|(offset, id)| {
                Document::parse_sprite(bytes_clone.clone(), *offset).map(|sprite| (*id, sprite))
            })
            .collect::<Result<HashMap<u16, SpriteBytes>, Error>>()?;

        Ok(Document {
            signature,
            count,
            sprites,
        })
    }
}

impl Document {
    fn parse_sprite(mut bytes: Bytes, offset: u32) -> Result<SpriteBytes, Error> {
        // FIXME: can this panic when we try to parse a malformed spr?
        bytes.advance(offset.try_into().unwrap());

        bytes.get_u8();
        bytes.get_u8();
        bytes.get_u8();

        let colored_bytes_count = bytes.get_u16_le() as u32;
        let mut bytes_put = 0;
        let mut colored_bytes_put = 0;
        let mut pixels = BytesMut::new();

        while colored_bytes_put < colored_bytes_count && bytes_put < SPRITE_BYTES {
            let transparent_pixels = bytes.get_u16_le();
            for _ in 0..transparent_pixels {
                if bytes_put >= SPRITE_BYTES {
                    break;
                }
                pixels.put_u32(0);
                bytes_put += 4;
            }
            let colored_pixels = bytes.get_u16_le() as u32;
            for _ in 0..colored_pixels {
                if bytes_put >= SPRITE_BYTES {
                    break;
                }
                pixels.put_u8(bytes.get_u8());
                pixels.put_u8(bytes.get_u8());
                pixels.put_u8(bytes.get_u8());
                pixels.put_u8(0xFF);
                bytes_put += 4;
            }
            colored_bytes_put += 4 + 3 * colored_pixels;
        }

        let bytes_padding = SPRITE_BYTES - bytes_put;

        if bytes_padding > 0 {
            pixels.put_slice(&vec![0; bytes_padding]);
        }

        pixels.to_vec().try_into()
    }
}
