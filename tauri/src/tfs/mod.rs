use crate::Project;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TfsProject {
    pub spr_path: PathBuf,
    pub dat_path: PathBuf,
    pub otb_path: PathBuf,
    pub otbm_path: PathBuf,
    pub houses_path: PathBuf,
    pub spawns_path: PathBuf,
}

impl From<TfsProject> for Project {
    fn from(value: TfsProject) -> Self {
        Project::TfsProject(value)
    }
}

mod detector;
mod loader;

pub use detector::*;
pub use loader::*;
