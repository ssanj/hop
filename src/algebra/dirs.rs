use std::path::PathBuf;
use crate::models::{LinkPair, HopEffect};

pub trait Dirs {
  fn get_hop_home(&self, path: &str) -> HopEffect<PathBuf>;
}
