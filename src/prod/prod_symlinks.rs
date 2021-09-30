use crate::models::{HopEffect, LinkPair, LinkTarget, Link};
use super::prod_models::Prod;

use crate::algebra::symlinks::SymLinks;
use std::fs::{self, DirEntry};

use std::path::PathBuf;
use std::io;

impl SymLinks for Prod {
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
