use crate::models::HopEffect;
use crate::algebra::directories::Directories;
use super::prod_models::Prod;

use std::path::PathBuf;

impl Directories for Prod {

  fn dir_exists(&self, dir_path: &PathBuf) -> HopEffect<bool> {
    Ok(dir_path.exists() && dir_path.is_dir())
  }

}
