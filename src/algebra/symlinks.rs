use std::path::{PathBuf, Path};
use crate::models::{LinkPair, HopEffect};

pub struct SymLink(pub PathBuf);

impl AsRef<Path> for SymLink {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

pub trait SymLinks {

    // fn read_link(&self, file_name: &PathBuf) -> HopEffect<String>;

    fn write_link(&self, symlink: &SymLink, target: &PathBuf) -> HopEffect<()>;

    // fn delete_link(&self, file_name: &PathBuf) -> HopEffect<()>;

    fn read_dir_links(&self, dir_path: &PathBuf) -> HopEffect<Vec<LinkPair>>;

    fn link_exists(&self, file_name: &PathBuf) -> HopEffect<bool>;
}
