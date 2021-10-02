use crate::models::HopEffect;
use std::path::PathBuf;

pub trait Directories {

  fn dir_exists(&self, dir_path: &PathBuf) -> HopEffect<bool>;
}
