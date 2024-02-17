use super::TfsProject;
use crate::detect::Detect;
use std::path::Path;

impl Detect for TfsProject {
    fn detect(directory: &Path) -> Option<Self> {
        todo!()
    }
}
