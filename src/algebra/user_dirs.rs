use crate::models::{HomeType, HopEffect};
use std::path::PathBuf;

pub trait UserDirs {
    fn get_hop_home(&self, path: &HomeType) -> HopEffect<PathBuf>;
}
