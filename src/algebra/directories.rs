use crate::models::HopEffect;
use std::path::Path;

pub trait Directories {
    fn dir_exists(&self, dir_path: &Path) -> HopEffect<bool>;
}
