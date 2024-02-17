use super::SkylessProject;
use crate::detect::Detect;
use std::path::Path;

impl Detect for SkylessProject {
    fn detect(directory: &Path) -> Option<Self> {
        todo!()
    }
}
