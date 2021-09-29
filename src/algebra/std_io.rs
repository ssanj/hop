use crate::models::HopEffect;

pub trait StdIO {
    fn println(&mut self, message: &str);
    fn eprintln(&mut self, message: &str);
    fn readln(&mut self) -> HopEffect<String>;
}

