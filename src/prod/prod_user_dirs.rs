use super::prod_models::Prod;
use crate::algebra::user_dirs::UserDirs;
use crate::models::HopEffect;
use dirs::home_dir;

use std::io;
use std::path::PathBuf;

impl UserDirs for Prod {
    fn get_hop_home(&self, path: &str) -> HopEffect<PathBuf> {
        Ok(get_home()?.join(path))
    }
}

fn get_home() -> HopEffect<PathBuf> {
    home_dir().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Could not get home directory"))
}
