use std::fmt;
use std::path::{Path, PathBuf};
use std::io;

pub type HopEffect<T> = io::Result<T>;

#[derive(Debug, Clone, PartialEq)]
pub struct Link(pub String);

impl Link {
    pub fn new(link: &str) -> Self {
        Self(link.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct LinkTarget(pub String);


impl LinkTarget {

    pub fn to_path_buf(&self) -> PathBuf {
        PathBuf::from(&self.0)
    }
}

#[derive(Debug, Clone)]
pub struct LinkPair {
    pub link: Link,
    pub target: LinkTarget
}

impl LinkPair {
  pub fn new(link: &str, target: &str) -> Self {
    LinkPair {
      link: Link(link.to_string()),
      target: LinkTarget(target.to_string())
    }
  }
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


impl AsRef<Path> for Link {
    fn as_ref(&self) -> &Path {
      self.0.as_ref()
    }
}

impl AsRef<Path> for LinkTarget {
    fn as_ref(&self) -> &Path {
      self.0.as_ref()
    }
}

impl fmt::Display for LinkTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for LinkPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.link, self.target)
    }
}
