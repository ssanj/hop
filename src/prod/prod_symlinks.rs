use super::prod_models::Prod;
use crate::models::{HopEffect, Link, LinkPair, LinkTarget};
use crate::program::{io_error, io_error_ex};

use crate::algebra::symlinks::{SymLink, SymLinks};
use std::fs::{self, DirEntry};

use std::io;
use std::os::unix::fs as nixfs;
use std::path::Path;

impl SymLinks for Prod {
    fn read_dir_links(&self, dir_path: &Path) -> HopEffect<Vec<LinkPair>> {
        get_links(dir_path)
    }

    fn write_link(&self, sym_link: &SymLink, target: &Path) -> HopEffect<()> {
        nixfs::symlink(target, sym_link)
    }

    fn link_exists(&self, sym_link: &Path) -> HopEffect<bool> {
        Ok(sym_link.exists())
    }

    fn delete_link(&self, dir_path: &Path, link_pair: &LinkPair) -> HopEffect<()> {
        let file_path = (dir_path).join(&link_pair.link);
        fs::remove_file(file_path)?;
        Ok(())
    }
}

//TODO: Refactor this
fn get_links(path: &Path) -> HopEffect<Vec<LinkPair>> {
    match fs::read_dir(path) {
        Ok(dir_it) => {
            let symlinks = dir_it
                .filter(|res| res.as_ref().map_or_else(|_| false, |d| is_symlink(d)))
                .map(|res| res.and_then(|entry| create_link_pair(&entry)))
                .collect::<Result<Vec<_>, io::Error>>()?; //sequence
            Ok(symlinks)
        },
        Err(e) => Err(io_error_ex(&format!("Could not read directory: {}", path.to_string_lossy()), e)),
    }
}

fn is_symlink(dir_entry: &DirEntry) -> bool {
    dir_entry
        .path()
        .symlink_metadata()
        .map_or_else(|_| false, |meta| meta.file_type().is_symlink())
}

fn create_link_pair(dir_entry: &DirEntry) -> HopEffect<LinkPair> {
    let link_path = &dir_entry.path();
    //Choose to display a lossy string
    let link = &dir_entry.file_name().to_string_lossy().to_string();

    let target_res = fs::read_link(link_path);
    match target_res {
        Ok(target) => Ok(LinkPair {
            link: Link(link.to_string()),
            target: LinkTarget(target.to_string_lossy().to_string()),
        }),
        Err(e) => Err(io_error(&format!(
            "Could not read link `{}` because: {}",
            link, e
        ))),
    }
}
