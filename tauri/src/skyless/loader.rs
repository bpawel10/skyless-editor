use super::SkylessProject;
use crate::{
    load::{Error, Load},
    transport::Transport,
};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
impl Load for SkylessProject {
    async fn load(&self, transport: Arc<impl Transport + Send + Sync>) -> Result<(), Error> {
        todo!()
    }
}
