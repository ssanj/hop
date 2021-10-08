use super::prod_models::Prod;
use crate::algebra::std_io::StdIO;
use std::io;

impl StdIO for Prod {
    fn println(&self, message: &str) {
        println!("{}", message)
    }

    fn readln(&self) -> io::Result<String> {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let line = buffer.lines().next().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Could not read stdin line"))?;
        Ok(line.to_owned())
    }
}
