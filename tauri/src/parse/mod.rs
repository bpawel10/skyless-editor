use std::io::{self, Read};

use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Error {
    Malformed,
    Io, // (IoError), FIXME: find a way to serialize std::io::Error
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io
    }
}

pub trait Parse<T>: Read + Sized {
    fn parse(self) -> Result<T, Error>;
}

pub mod dat;
pub mod otb;
pub mod otbm;
pub mod spr;
