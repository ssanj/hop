use super::prod_models::Prod;
use crate::algebra::user_dirs::UserDirs;
use crate::models::{HomeType, HopEffect};
use crate::program::{io_error, io_error_ex_nested};
use dirs::home_dir;

use std::{fs, io};
use std::path::PathBuf;

impl UserDirs for Prod {
    fn get_hop_home(&self, home_type: &HomeType) -> HopEffect<PathBuf> {
        let hop_home = match home_type {
            HomeType::Relative(path) => get_home()?.join(path),
            HomeType::Absolute(absolute_path) => PathBuf::from(absolute_path),
        };

        match fs::metadata(&hop_home) {
            Ok(dir) =>
                if dir.is_dir() {
                    Ok(hop_home)
                } else {
                    Err(io_error(&format!("{} is not a directory", &hop_home.to_string_lossy())))
                },
            Err(e1) => {
                //hop_home is not a directory, try and create it
                match fs::create_dir_all(&hop_home) {
                    Ok(_) => Ok(hop_home),
                    Err(e2) => Err(io_error_ex_nested(&format!("Could not create dir: {}", &hop_home.to_string_lossy()), e2, e1)),
                }
            }
        }
    }
}

fn get_home() -> HopEffect<PathBuf> {
    home_dir().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Could not get home directory"))
}
