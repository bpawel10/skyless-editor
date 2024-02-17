use crate::load::Load;
use std::path::Path;

pub trait Detect: Load + Sized {
    fn detect(directory: &Path) -> Option<Self>;
}
