use std::error::Error;
use std::io;
use std::path::PathBuf;
use std::fs;
use dirs::home_dir;


fn main() -> Result<(), Box<dyn Error>>{
    println!("~Hop~");
    let home = get_home()?;
    let home_entries = get_hop_home(home)?;
    for dir in home_entries {
        let file_name = get_file_name(dir).unwrap_or_else(|| "--unknown--".to_string());
        println!("{}", file_name);
    }

    Ok(())
}

fn get_file_name(dir: PathBuf) -> Option<String> {
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
