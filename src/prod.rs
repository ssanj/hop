use crate::algebra::std_io;
use crate::algebra::user_dirs;
use crate::algebra::symlinks;
use dirs::home_dir;
use std::fs::{self, DirEntry};
// use std::error::Error;

use std::path::PathBuf;
use crate::models::{HopEffect, LinkPair, LinkTarget, Link};
use std::io;

pub struct Prod;

impl std_io::StdIO for Prod {
    fn println(&mut self, message: &str) {
        println!("{}", message)
    }

    fn eprintln(&mut self, message: &str) {
        eprintln!("{}", message)
    }

    fn readln(&mut self) -> io::Result<String> {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let line = buffer.lines().next().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Could not read stdin line"))?;
        Ok(line.to_owned())
    }
}

impl user_dirs::UserDirs for Prod {
  fn get_hop_home(&self, path: &str) -> HopEffect<PathBuf> {
    Ok(get_home()?.join(path))
  }
}

impl symlinks::SymLinks for Prod {
    fn read_dir_links(&self, dir_path: &PathBuf) -> HopEffect<Vec<LinkPair>> {
        get_links(dir_path)
    }
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
    //what if the file is not a link? We should filter those out
    let target_res = fs::read_link(link_path);

    target_res.map(|target| LinkPair{ link: Link(link), target: LinkTarget( target.to_string_lossy().to_string()) })
}

fn get_home() -> HopEffect<PathBuf> {
    home_dir().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Could not get home directory"))
}
