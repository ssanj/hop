use crate::models::HopEffect;

pub trait StdIO {
    fn println(&self, message: &str);
    fn readln(&self) -> HopEffect<String>;
}

