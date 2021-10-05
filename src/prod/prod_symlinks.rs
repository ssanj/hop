use crate::models::{HopEffect, LinkPair, LinkTarget, Link};
use crate::io_error;
use super::prod_models::Prod;

use crate::algebra::symlinks::{SymLinks, SymLink, SymLinkDeleteStatus};
use std::fs::{self, DirEntry};


use std::path::PathBuf;
use std::io;
use std::os::unix::fs as nixfs;

impl SymLinks for Prod {
    fn read_dir_links(&self, dir_path: &PathBuf) -> HopEffect<Vec<LinkPair>> {
        get_links(dir_path)
    }

    fn write_link(&self, symlink: &SymLink, target: &PathBuf) -> HopEffect<()> {
        nixfs::symlink(target, symlink)
    }

    fn link_exists(&self, symLink: &PathBuf) -> HopEffect<bool> {
        Ok(symLink.exists())
    }

    fn delete_link(&self, dir_path: &PathBuf, linkPair: &LinkPair) -> HopEffect<SymLinkDeleteStatus> {
        let link_pairs = get_links(&dir_path)?;
        match link_pairs.iter().find(|lp| lp.link == linkPair.link) {
            Some(pair) => {
                let prompt_message = format!("Are you sure you want to delete {} which links to {} ?", pair.link, pair.target);
                let no_action = || Ok(SymLinkDeleteStatus::Aborted(pair.clone()));
                let yes_action = || {
                    let file_path = (&dir_path.clone()).join(&pair.link);
                    fs::remove_file(file_path)?;
                    Ok(SymLinkDeleteStatus::Success(linkPair.clone()))
                    // Ok(println!("Removed link {} which pointed to {}", &pair.link, &pair.target))
                };

                prompt_user(&prompt_message, yes_action, no_action)
            },

            None => Ok(SymLinkDeleteStatus::NotFound(linkPair.clone()))// Err(io_error(&format!("Could not find link named:{} to delete", &linkPair.link)))
       }
    }
}

fn prompt_user<Y, N, T>(message: &str, yes_action: Y, no_action: N) -> HopEffect<T> where
    Y: FnOnce() -> HopEffect<T>,
    N: FnOnce() -> HopEffect<T>
{
    println!("{}", message);
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let response = buffer.lines().next().ok_or(io_error("Could not retrieve lines from stdio"))?;
    match response {
        "Y" | "y"  => yes_action(),
        _ => no_action()
    }
}

// fn delete(hop_home: &PathBuf, link: Link) -> HopEffect<()> {

//     let result = match get_links(hop_home) {
//         Ok(link_pairs) => {
//            match link_pairs.iter().find(|lp| lp.link == link) {
//             Some(pair) => {
//                 let prompt_message = format!("Are you sure you want to delete {} which links to {} ?", pair.link, pair.target);
//                 let no_action = || Ok(println!("Aborting delete of {}", pair.link));
//                 let yes_action = || {
//                     let file_path = (hop_home.clone()).join(&link);
//                     fs::remove_file(file_path)?;
//                     Ok(println!("Removed link {} which pointed to {}", &link, &pair.target))
//                 };

//                 prompt_user(&prompt_message, yes_action, no_action)?
//             },

//             None => eprintln!("Could not find link named:{} to delete", link)
//            }
//         },

//         Err(e) => println!("Could not retrieve links: {}", e)
//     };
//     Ok(result)
// }

fn get_links(path: &PathBuf) -> HopEffect<Vec<LinkPair>> {
    let x = fs::read_dir(path)?;
    x.map(|res| res.and_then(|entry| create_link_pair(entry)))
    .collect::<Result<Vec<_>, io::Error>>() //sequence
}

fn create_link_pair(dir_entry: DirEntry) -> HopEffect<LinkPair> {
    let link_path = dir_entry.path();
    //Choose to display a lossy string
    let link = dir_entry.file_name().to_string_lossy().to_string();
    //what if the file is not a link? We should filter those out
    let target_res = fs::read_link(link_path);

    target_res.map(|target| LinkPair{ link: Link(link), target: LinkTarget( target.to_string_lossy().to_string()) })
}
