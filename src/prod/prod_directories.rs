use super::prod_models::Prod;
use crate::algebra::directories::Directories;
use crate::models::HopEffect;

use std::path::Path;

impl Directories for Prod {
    fn dir_exists(&self, dir_path: &Path) -> HopEffect<bool> {
        Ok(dir_path.exists() && dir_path.is_dir())
    }
}
