use std::fmt;
use std::error::Error;
use std::io;
use std::path::PathBuf;
use std::fs::{self, DirEntry};
use dirs::home_dir;
use clap::{App, Arg};


//todo: How do we test any?
fn main() -> Result<(), Box<dyn Error>>{
    let matches = App::new("Hop")
        .version("0.1.0")
        .author("Sanj Sahayam")
        .about("Hop to frequently used directories")
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .help("Lists hoppable directories")
        )
        .arg(
            Arg::with_name("jump")
                .short("j")
                .long("jump")
                .value_name("NAME")
                .help("Jump to a named directory")
                .takes_value(true)
        )
        .get_matches();

    let hop_home = get_home()?.join(".hop");

    let program =
        if matches.is_present("list") {
            let _result = list_links(&hop_home);
            ()
        } else if let Some(j) = matches.value_of("jump") {
            let _result = jump_to(&hop_home, Link(j.to_string()));
            ()
        } else {
            println!("Not sure what you want! See usage");
        };

    Ok(program)
}

fn jump_to(hop_home: &PathBuf, link: Link) -> Result<(), io::Error> {
    let result = match get_links(hop_home) {
        Ok(link_pairs) => {
            let mut found = false;
            for lp in link_pairs {
                if link == lp.link {
                    println!("{}", lp.target);
                    found = true;
                }
            }

            if found {
                ()
            } else {
                println!("Could not find link: {}", link)
            }



        },

        Err(e) => println!("Could not retrieve links: {}", e)
    };

    Ok(result)

}

fn list_links(hop_home: &PathBuf) -> Result<(), io::Error> {
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

#[derive(Debug, PartialEq)]
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


fn get_links(path: &PathBuf) -> Result<Vec<LinkPair>, io::Error> {
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
