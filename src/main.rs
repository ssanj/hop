use std::error::Error;
use std::path::PathBuf;
use std::fs::{self, DirEntry};
use dirs::home_dir;
use clap::{App, Arg};
use models::{Link, LinkPair, LinkTarget};
use std::path::Path;
use std::os::unix::fs as nixfs;
use std::io;

mod models;

type HopEffect<T> = io::Result<T>;

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
        )
        .arg(
            Arg::with_name("mark")
                .short("m")
                .long("mark")
                .value_names(&["NAME", "PATH"])
                .help("Mark a named directory")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("delete")
                .short("d")
                .long("delete")
                .value_name("NAME")
                .help("Delete a named directory")
                .takes_value(true)
        );

     let mut app2 = app.clone(); //we need this close to display usage on error
     let matches = app.get_matches();

    let hop_home = get_home()?.join(".hop");

    let program =
        if matches.is_present("list") {
            let _result = list_links(&hop_home);
            ()
        } else if let Some(j) = matches.value_of("jump") {
            let _result = jump_to(&hop_home, Link(j.to_string()));
            ()
        } else if let Some(m) = matches.values_of("mark") {
            let mut values = m.clone();
            let link_op = values.next();
            let target_op = values.next();

            //We can use something like a command pattern here.
            match (link_op, target_op) {
                (Some(link), Some(target)) => {
                    let target_path = Path::new(target);
                    if target_path.exists(){
                        if target_path.is_dir() {
                            let _result = mark(&hop_home, LinkPair::new(link, target));
                            //we should dump out the error on all these
                            ()
                        } else {
                            eprintln!("{} is not a directory.", target)
                        }
                    } else {
                        eprintln!("{} does not exist or you do not have permission to it.", target)
                    }
                },
                _ => println!("Need both link and target to create a mark")
            }

        } else if let Some(d) = matches.value_of("delete") {
            let _result = delete(&hop_home, Link(d.to_string()));
            ()
        } else {
            let _result = app2.print_help();
            ()
        };

    Ok(program)
}

// #[allow(dead_code)]
// fn process_links<F, G, A>(hop_home: &PathBuf, f: F) -> Result<(), io::Error> where
//     F: Fn(Vec<LinkPair>)
//  {
//     let result = match get_links(hop_home) {
//         Ok(link_pairs) => f(link_pairs),
//         Err(e) => eprintln!("Could not retrieve links: {}", e)
//     };

//     Ok(result)
// }

fn io_error(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, message.clone())
}

fn prompt_user<Y, N, T>(message: &str, yes_action: Y, no_action: N) -> HopEffect<T> where
    Y: FnOnce() -> HopEffect<T>,
    N: FnOnce(String) -> HopEffect<T>
{
    println!("{}", message);
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let response = buffer.lines().next().ok_or(io_error("Could not retrieve lines from stdio"))?;
    match response {
        "Y" | "y"  => yes_action(),
        other => no_action(other.to_string())
    }
}

fn delete(hop_home: &PathBuf, link: Link) -> HopEffect<()> {

    let result = match get_links(hop_home) {
        Ok(link_pairs) => {
           match link_pairs.iter().find(|lp| lp.link == link) {
            Some(pair) => {
                let prompt_message = format!("Are you sure you want to delete {} which links to {} ?", pair.link, pair.target);
                let no_action = |error| Ok(println!("Aborting delete of {} because you said: {}", pair.link, error));
                let yes_action = || {
                    let file_path = (hop_home.clone()).join(&link);
                    fs::remove_file(file_path)?;
                    Ok(println!("Removed link {} which pointed to {}", &link, &pair.target))
                };

                prompt_user(&prompt_message, yes_action, no_action)?
            },

            None => eprintln!("Could not find link named:{} to delete", link)
           }
        },

        Err(e) => println!("Could not retrieve links: {}", e)
    };
    Ok(result)
}

fn mark(hop_home: &PathBuf, pair: LinkPair) -> HopEffect<()> {
    println!("You said mark with {} for {}", pair.link, pair.target);
    let mut symlink_path = hop_home.clone();
    symlink_path.push(pair.link);

    nixfs::symlink(pair.target, symlink_path)
}

fn jump_to(hop_home: &PathBuf, link: Link) -> HopEffect<()> {
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

fn list_links(hop_home: &PathBuf) -> HopEffect<()> {
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


fn get_links(path: &PathBuf) -> HopEffect<Vec<LinkPair>> {
    let x = fs::read_dir(path)?;
    x.map(|res| res.and_then(|entry| create_link_pair(entry)))
    .collect::<Result<Vec<_>, io::Error>>() //sequence
}

fn create_link_pair(dir_entry: DirEntry) -> HopEffect<LinkPair> {
    let link_path = dir_entry.path();
    //Choose to display a lossy string
    let link = dir_entry.file_name().to_string_lossy().to_string();
    let target_res = fs::read_link(link_path);

    target_res.map(|target| LinkPair{ link: Link(link), target: LinkTarget( target.to_string_lossy().to_string()) })
}
