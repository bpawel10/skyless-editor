use crate::project::Project;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SkylessProject {
    pub assets_path: PathBuf,
    pub map_path: PathBuf,
}

impl From<SkylessProject> for Project {
    fn from(value: SkylessProject) -> Self {
        Project::SkylessProject(value)
    }
}

mod detector;
mod loader;

pub use detector::*;
pub use loader::*;
