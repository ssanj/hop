use std::fmt;
use std::error::Error;
// use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use dirs::home_dir;

enum HopError {
    HomeDirNotFound,
    CouldNotCreateHopDir,
    CouldNotReadFromHopDir,
    CouldNotWriteToHopDir
}

impl fmt::Debug for HopError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

       let debug_info = match self {
            HopError::HomeDirNotFound => "Hop Error D: Could not find your home directory",
            HopError::CouldNotCreateHopDir => "Hop Error D: Could not create the .hop directory",
            HopError::CouldNotReadFromHopDir => "Hop Error D: Could not read from the .hop directory",
            HopError::CouldNotWriteToHopDir => "Hop Error D: Could not write to the .hop directory"
        };

        write!(f,"{}", debug_info)
    }
}

impl fmt::Display for HopError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"Hop Error2: {:?}",self)
    }
}

impl Error for HopError {
    fn description(&self) -> &str {
        match self {
            HopError::HomeDirNotFound => "Hop Error: Could not find your home directory",
            HopError::CouldNotCreateHopDir => "Hop Error: Could not create the .hop directory",
            HopError::CouldNotReadFromHopDir => "Hop Error: Could not read from the .hop directory",
            HopError::CouldNotWriteToHopDir => "Hop Error: Could not write to the .hop directory"
        }
    }
}

fn main() -> Result<(), HopError>{
    let home = get_home()?;
    println!("home directory: {:?}", home);
    Ok(())
}

fn get_home() -> Result<PathBuf, HopError> {
    None.ok_or_else(|| HopError::HomeDirNotFound)
}
