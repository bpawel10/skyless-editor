use crate::{parse, transport::Transport};
use async_trait::async_trait;
use serde::Serialize;
use std::{io, sync::Arc};
use tauri::State;

#[derive(Serialize)]
pub enum Error {
    Something, // TODO:
    Parse(parse::Error),
}

impl From<parse::Error> for Error {
    fn from(value: parse::Error) -> Self {
        Error::Parse(value)
    }
}

// FIXME: why do we need that? it should be implicit...
impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Into::<parse::Error>::into(value).into()
    }
}

#[async_trait]
pub trait Load {
    async fn load(
        &self,
        transport: Arc<impl Transport + Send + Sync + 'static>,
    ) -> Result<(), Error>;
}
