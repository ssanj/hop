use std::error::Error;
use std::io;
use std::path::PathBuf;
use std::fs::{self, DirEntry};
use dirs::home_dir;
use clap::{App, Arg};
use models::{Link, LinkPair, LinkTarget};

mod models;


//todo: How do we test any?
fn main() -> Result<(), Box<dyn Error>>{
    let app = App::new("Hop")
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
        );

     let mut app2 = app.clone();
     let matches = app.get_matches();

    let hop_home = get_home()?.join(".hop");

    let program =
        if matches.is_present("list") {
            let _result = list_links(&hop_home);
            ()
        } else if let Some(j) = matches.value_of("jump") {
            let _result = jump_to(&hop_home, Link(j.to_string()));
            ()
        } else {
            let _result = app2.print_help();
            ()
        };

    Ok(program)
}

fn jump_to(hop_home: &PathBuf, link: Link) -> Result<(), io::Error> {
    let result = match get_links(hop_home) {
        Ok(link_pairs) => {
            match link_pairs.iter().find(|&lp| lp.link == link) {
                Some(found_lp) => println!("{}", found_lp.target),
                None => println!("Could not find link: {}", link)
            }
        },

        Err(e) => eprintln!("Could not retrieve links: {}", e)
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
