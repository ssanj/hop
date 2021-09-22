use std::fmt;
use std::error::Error;
use std::io;
use std::path::PathBuf;
use std::fs;
use dirs::home_dir;

enum HopError {
    CouldNotReadFromHopDir,
}

impl fmt::Debug for HopError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

       let debug_info = match self {
            HopError::CouldNotReadFromHopDir => "Hop Error D: Could not read from the .hop directory",
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
            HopError::CouldNotReadFromHopDir => "Hop Error: Could not read from the .hop directory",
        }
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    println!("~Hop~");
    let home = get_home()?;
    let home_entries = get_hop_home(home)?;
    for dir in home_entries {
        let file_name = do_blah(dir).unwrap_or_else(|| "--unknown--".to_string());
        println!("{}", file_name);
    }

    Ok(())
}

fn do_blah(dir: PathBuf) -> Option<String> {
    Some(dir.file_name()?.to_str()?.to_string())
}

fn get_home() -> Result<PathBuf, io::Error> {
    home_dir().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Could not get home directory"))
}

fn get_hop_home(home: PathBuf) -> Result<Vec<PathBuf>, io::Error> {
    let mut hop_home  = home.clone();
    hop_home.push(".hop");

    if hop_home.is_dir() {
        let entries = fs::read_dir(hop_home)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;

        Ok(entries)
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Could not get ~/.hop directory"))
    }


}
