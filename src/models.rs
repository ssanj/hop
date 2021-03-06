use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

pub type HopEffect<T> = io::Result<T>;

#[derive(Debug, Clone, PartialEq)]
pub struct Link(pub String);

impl Link {
    pub fn new(link: &str) -> Self {
        Self(link.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LinkTarget(pub String);

impl LinkTarget {
    pub fn to_path_buf(&self) -> PathBuf {
        PathBuf::from(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LinkPair {
    pub link: Link,
    pub target: LinkTarget,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HomeType {
    Relative(String),
    Absolute(String),
}

impl LinkPair {
    pub fn new(link: &str, target: &str) -> Self {
        LinkPair {
            link: Link(link.to_string()),
            target: LinkTarget(target.to_string()),
        }
    }
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for HomeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path_type = match self {
            HomeType::Absolute(path) => format!("Absolute({})", path),
            HomeType::Relative(path) => format!("Relative({})", path),
        };

        write!(f, "{}", path_type)
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
