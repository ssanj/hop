use std::path::PathBuf;
use crate::models::{LinkPair, HopEffect};

pub trait SymLinks {

    // fn read_link(&self, file_name: &PathBuf) -> HopEffect<String>;

    // fn write_link(&self, file_name: &PathBuf, target: &PathBuf) -> HopEffect<()>;

    // fn delete_link(&self, file_name: &PathBuf) -> HopEffect<()>;

    fn read_dir_links(&self, dir_path: &PathBuf) -> HopEffect<Vec<LinkPair>>;

    // fn link_exists(&self, file_name: &PathBuf) -> HopEffect<bool>;
}
