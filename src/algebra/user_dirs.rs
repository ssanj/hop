use std::path::PathBuf;
use crate::models::HopEffect;

pub trait UserDirs {
  fn get_hop_home(&self, path: &str) -> HopEffect<PathBuf>;
}
