use crate::{
    load::{Error, Load},
    skyless::SkylessProject,
    tfs::TfsProject,
    transport::Transport,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize)]
pub enum Project {
    SkylessProject(SkylessProject),
    TfsProject(TfsProject),
}

// TODO: use some macro to auto-impl this
#[async_trait]
impl Load for Project {
    async fn load(
        &self,
        transport: Arc<impl Transport + Send + Sync + 'static>,
    ) -> Result<(), Error> {
        match self {
            Project::SkylessProject(project) => project.load(transport),
            Project::TfsProject(project) => project.load(transport),
        }
        .await
    }
}
