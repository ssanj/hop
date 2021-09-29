use std::path::PathBuf;
use crate::models::{LinkPair, HopEffect};

pub trait SymLinks {

    fn read_link(file_name: &PathBuf) -> HopEffect<String>;

    fn write_link(file_name: &PathBuf, target: &PathBuf) -> HopEffect<()>;

    fn delete_link(file_name: &PathBuf) -> HopEffect<()>;

    fn read_dir_links(dir_path: &PathBuf) -> HopEffect<Vec<LinkPair>>;

    fn link_exists(file_name: &PathBuf) -> HopEffect<bool>;
}
