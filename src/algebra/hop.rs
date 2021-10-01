use crate::models::{HopEffect, LinkPair, Link};

use super::{user_dirs::UserDirs, std_io::StdIO, symlinks::SymLinks};

/// The data required to run hop
pub struct HopProgram<T>{
    pub value: T,
    pub cfg_dir: String
}

impl <T> HopProgram<T>
  where
    T : UserDirs + StdIO + SymLinks
  {

  pub fn list_links(&mut self) -> HopEffect<()> {
    let entries = self.get_link_pairs()?;
    entries.iter().for_each(|lp| self.value.println(&format!("{}", lp.link)));
    Ok(())
  }

  pub fn jump_target(&mut self, link: Link) -> HopEffect<()> {
    let entries = self.get_link_pairs()?;
    let result = match entries.iter().find(|&lp| lp.link == link) {
        Some(found_lp) => println!("{}", found_lp.target),
        None => println!("Could not find link: {}", link)
    };

    Ok(result)
  }

  //Ideally we just get this "capability", as it makes it easier to test
  //This capability can depend on UserDirs + StdIO + SymLinks
  fn get_link_pairs(&mut self) -> HopEffect<Vec<LinkPair>> {
      let hop_home_dir = self.value.get_hop_home(&self.cfg_dir)?;
      let entries = self.value.read_dir_links(&hop_home_dir)?;

      Ok(entries.to_vec())
  }

}

#[cfg(test)]
mod tests {
    use crate::algebra::{std_io::StdIO, symlinks::SymLinks, user_dirs::UserDirs};
    use crate::models::{HopEffect, LinkPair};
    use super::HopProgram;

    use std::path::PathBuf;
    use std::io;

    struct Test<'a> {
      out: &'a mut Vec<String>,
      get_hop_home: Option<String>,
      read_dir_links: Result<Vec<LinkPair>, String>
    }

    impl StdIO for Test<'_> {

      fn println(&mut self, message: &str) {
        (self.out).push(message.to_string())
      }

      fn eprintln(&mut self, message: &str) {
        todo!()
      }

      fn readln(&mut self) -> HopEffect<String> {
        todo!()
      }
    }

    impl UserDirs for Test<'_> {
      fn get_hop_home(&self, path: &str) -> HopEffect<PathBuf> {
        match &self.get_hop_home {
          Some(error) => Err(io::Error::new(io::ErrorKind::Other, error.to_string())),
          None =>  Ok(PathBuf::from("/xyz/.your-hop"))
        }

      }
    }

    impl SymLinks for Test<'_> {
      fn read_dir_links(&self, dir_path: &PathBuf) -> HopEffect<Vec<LinkPair>> {

        match &self.read_dir_links {
          Ok(links) => Ok(links.to_vec()),
          Err(error) => Err(io::Error::new(io::ErrorKind::Other, error.to_string()))
        }
      }
    }


    #[test]
    fn list_links_success() {
      let mut output = vec![];
      let read_links =
        vec![
          LinkPair::new("myLink", "/my/path/to/link"),
          LinkPair::new("myOtherLink", "/my/path/to/Otherlink")
        ];

      let test_val = Test { out: &mut output, get_hop_home: None, read_dir_links: Ok(read_links) };
      let mut program = HopProgram { value: test_val, cfg_dir: ".hop".to_string() };
      match program.list_links() {
        Ok(_) => assert_eq!(&vec!["myLink".to_string(), "myOtherLink".to_string()], &output),
        Err(e) => panic!("{}: Expected an Ok but got err", e)
      }
    }

    #[test]
    fn list_links_home_dir_failure() {
      let mut output = vec![];
      let cfg_dir = ".blah".to_string();
      let value = Test { out: &mut output, get_hop_home: Some("Failed to get home dir".to_string()), read_dir_links: Ok(vec![]) };

      let mut program = HopProgram { value, cfg_dir };

      match program.list_links() {
        Err(e) => assert_eq!(e.to_string(), "Failed to get home dir"),
        Ok(_) => panic!("Expected an Err but got Ok")
      }
    }

    #[test]
    fn list_links_read_links_failure() {
      let mut output = vec![];
      let cfg_dir = ".blah".to_string();
      let value = Test { out: &mut output, get_hop_home: None, read_dir_links: Err("Failed to read links".to_string()) };

      let mut program = HopProgram { value, cfg_dir };

      match program.list_links() {
        Err(e) => assert_eq!(e.to_string(), "Failed to read links"),
        Ok(_) => panic!("Expected an Err but got Ok")
      }
    }

    #[test]
    fn list_links_read_links_no_result() {
      let mut output: Vec<String> = vec![];
      let cfg_dir = ".blah".to_string();
      let value = Test { out: &mut output, get_hop_home: None, read_dir_links: Ok(vec![]) };

      let mut program = HopProgram { value, cfg_dir };

      match program.list_links() {
        Ok(_) => assert!(output.is_empty(), "Expected output to be empty but got: {:?}" ,output),
        Err(e) => panic!("{}: Expected an Ok but got err", e)
      }
    }
}
