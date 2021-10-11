use std::path::{PathBuf, Path};
use crate::models::{LinkPair, HopEffect};

pub struct SymLink(pub PathBuf);


impl AsRef<Path> for SymLink {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

pub trait SymLinks {

    fn write_link(&self, symlink: &SymLink, target: &Path) -> HopEffect<()>;

    fn delete_link(&self, dir_path: &Path, link_pair: &LinkPair) -> HopEffect<()>;

    fn read_dir_links(&self, dir_path: &Path) -> HopEffect<Vec<LinkPair>>;

    fn link_exists(&self, file_name: &Path) -> HopEffect<bool>;
}
