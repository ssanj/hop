use std::path::PathBuf;
use crate::models::{LinkPair, HopEffect};

pub trait Dirs {
  fn read_dir(path: &PathBuf) -> dyn Iterator<Item =  LinkPair>;

  fn get_home_dir() -> HopEffect<PathBuf>;

  fn in_home_dir(path: &PathBuf) -> HopEffect<PathBuf>;
}
