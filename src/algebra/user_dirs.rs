use crate::models::HopEffect;
use std::path::PathBuf;

pub trait UserDirs {
    fn get_hop_home(&self, path: &str) -> HopEffect<PathBuf>;
}
