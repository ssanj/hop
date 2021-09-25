use std::fmt;


#[derive(Debug, PartialEq)]
pub struct Link(pub String);

#[derive(Debug)]
pub struct LinkTarget(pub String);

#[derive(Debug)]
pub struct LinkPair {
    pub link: Link,
    pub target: LinkTarget
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
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
