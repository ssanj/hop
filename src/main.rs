use std::fmt;
use std::error::Error;
use std::io;
use std::path::PathBuf;
use std::fs::{self, DirEntry};
use dirs::home_dir;


//todo: How do we test any?
fn main() -> Result<(), Box<dyn Error>>{
    let hop_home = get_home()?.join(".hop");

    let result = match get_links(hop_home) {
        Ok(link_pairs) => {
            for lp in link_pairs {
                println!("{}", lp.link)
            }
        },

        Err(e) => println!("Could not retrieve links: {}", e)
    };

    Ok(result)
}

#[allow(dead_code)]
fn get_file_name(dir: PathBuf) -> Option<String> {
    Some(dir.file_name()?.to_str()?.to_string())
}

fn get_home() -> Result<PathBuf, io::Error> {
    home_dir().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Could not get home directory"))
}

#[derive(Debug)]
struct Link(String);

#[derive(Debug)]
struct LinkTarget(String);

#[derive(Debug)]
struct LinkPair {
    link: Link,
    target: LinkTarget
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for LinkTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for LinkPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.link, self.target)
    }
}


fn get_links(path: PathBuf) -> Result<Vec<LinkPair>, io::Error> {
    let x = fs::read_dir(path)?;
    x.map(|res| res.and_then(|entry| create_link_pair(entry)))
    .collect::<Result<Vec<_>, io::Error>>() //traverse
}

fn create_link_pair(dir_entry: DirEntry) -> io::Result<LinkPair> {
    let link_path = dir_entry.path();
    //Choose to display a lossy string
    let link = dir_entry.file_name().to_string_lossy().to_string();
    let target_res = fs::read_link(link_path);

    target_res.map(|target| LinkPair{ link: Link(link), target: LinkTarget( target.to_string_lossy().to_string()) })
}

#[allow(dead_code)]
fn new_io_error(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, message)
}

#[allow(dead_code)]
fn get_hop_home2(hop_home: &mut PathBuf) -> Result<Vec<PathBuf>, io::Error> {
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
