use crate::{io_error, models::{HopEffect, LinkPair, Link}};

use super::{std_io::StdIO, symlinks::SymLinks, symlinks::SymLink, user_dirs::UserDirs, directories::Directories};
use std::path::PathBuf;
use std::fs;

/// The data required to run hop
pub struct HopProgram<T>{
    pub value: T,
    pub cfg_dir: String
}

impl <T> HopProgram<T>
  where
    T : UserDirs + StdIO + SymLinks + Directories
  {

  pub fn list_links(&mut self) -> HopEffect<()> {
    let entries = self.get_link_pairs()?;
    entries.iter().for_each(|lp| self.value.println(&format!("{}", lp.link)));
    Ok(())
  }

  pub fn jump_target(&mut self, link: Link) -> HopEffect<()> {
    let entries = self.get_link_pairs()?;
    let result = match entries.iter().find(|&lp| lp.link == link) {
        Some(found_lp) => self.value.println(&format!("{}", found_lp.target)),
        None => self.value.println(&format!("Could not find link: {}", link))
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

  pub fn mark_dir(&mut self, pair: LinkPair) -> HopEffect<()> {
    let hop_home = self.value.get_hop_home(&self.cfg_dir)?;
    let symlink_path = (hop_home.clone()).join(&pair.link);

    let target_path = pair.target.to_path_buf();

    //TODO: Send in a LinkTarget
    if self.value.dir_exists(&target_path)? {
      //TODO: Send in a SymLink
      if self.value.link_exists(&symlink_path)? {
        Err(io_error(&format!("A link named `{}` already exists. Aborting mark creation.", &pair.link)))
      } else {
        self.value.write_link(&SymLink(symlink_path), &pair.target.to_path_buf())
      }
    } else {
      Err(io_error(&format!("A directory named `{}` does not exist or you do not have permission to it.", &pair.target)))
    }
  }

  // pub fn delete(&mut self, link: Link) -> HopEffect<()> {
  //   let link_pairs = self.get_link_pairs()?;

  //   match link_pairs.iter().find(|lp| lp.link == link) {
  //    Some(pair) => {
  //        let prompt_message = format!("Are you sure you want to delete {} which links to {} ?", pair.link, pair.target);

  //        let no_action = || {
  //         &self.value.println(&format!("Aborting delete of {}", pair.link));
  //         Ok(())
  //       };

  //        let yes_action = || {
  //            let hop_home = &self.value.get_hop_home(&self.cfg_dir)?;
  //            let file_path = (hop_home.clone()).join(&link);
  //            fs::remove_file(file_path)?;
  //            // &self.value.println(&format!("Removed link {} which pointed to {}", &link, &pair.target));
  //            Ok(())
  //        };

  //        self.prompt_user(&prompt_message, yes_action, no_action)
  //    },

  //    None => Err(io_error(&format!("Could not find link named:{} to delete", link)))
  //   }
  // }

  fn prompt_user<Y, N, R>(&self, message: &str, yes_action: Y, no_action: N) -> HopEffect<R> where
      Y: FnOnce() -> HopEffect<R>,
      N: FnOnce() -> HopEffect<R>
  {
      // self.value.println(message);
      // let buffer = self.value.readln()?;
      // let response = buffer.lines().next().ok_or(io_error("Could not retrieve lines from stdio"))?;
      // match response {
      //     "Y" | "y"  => yes_action(),
      //     _ => no_action()
      // }
      todo!()
  }

}

//TODO: Move this out into a separate module
#[cfg(test)]
mod tests {
    use crate::algebra::{std_io::StdIO, user_dirs::UserDirs, directories::Directories};
    use crate::algebra::symlinks::{SymLinks, SymLink};
    use crate::models::{HopEffect, LinkPair, Link};
    use super::HopProgram;

    use std::path::PathBuf;
    use std::io;

    //TODO: consider prefixing these fields, so as not to confuse them with the real implementation
    struct Test<'a> {
      out: &'a mut Vec<String>,
      get_hop_home: Option<String>,
      read_dir_links: Result<Vec<LinkPair>, String>,
      dir_exists: bool,
      link_exists: bool,
      write_link: Option<String>
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

      fn write_link(&self, symlink: &SymLink, target: &PathBuf) -> HopEffect<()> {
        match &self.write_link {
          Some(error) => Err(io::Error::new(io::ErrorKind::Other, error.to_string())),
          None => Ok(())
        }
      }

      fn link_exists(&self, file_name: &PathBuf) -> HopEffect<bool> {
        Ok(self.link_exists)
      }

      fn delete_link(&self, dir_path: &PathBuf, linkPair: &LinkPair) -> HopEffect<()> {
        todo!()
      }

    }

    impl Directories for Test<'_> {
      fn dir_exists(&self, dir_path: &PathBuf) -> HopEffect<bool> {
        Ok(self.dir_exists)
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

      let test_val = Test { out: &mut output, get_hop_home: None, read_dir_links: Ok(read_links), dir_exists: true, link_exists: false, write_link: None };
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
      let value = Test { out: &mut output, get_hop_home: Some("Failed to get home dir".to_string()), read_dir_links: Ok(vec![]), dir_exists: true, link_exists: false, write_link: None };

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
      let value = Test { out: &mut output, get_hop_home: None, read_dir_links: Err("Failed to read links".to_string()), dir_exists: true, link_exists: false, write_link: None };

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
      let value = Test { out: &mut output, get_hop_home: None, read_dir_links: Ok(vec![]), dir_exists: true, link_exists: false, write_link: None };

      let mut program = HopProgram { value, cfg_dir };

      match program.list_links() {
        Ok(_) => assert!(output.is_empty(), "Expected output to be empty but got: {:?}" ,output),
        Err(e) => panic!("{}: Expected an Ok but got err", e)
      }
    }

    #[test]
    fn jump_target_success() {
      let mut output = vec![];
      let read_links =
        vec![
          LinkPair::new("myLink", "/my/path/to/link"),
          LinkPair::new("myOtherLink", "/my/path/to/Otherlink")
        ];

      let test_val = Test { out: &mut output, get_hop_home: None, read_dir_links: Ok(read_links), dir_exists: true, link_exists: false, write_link: None };
      let mut program = HopProgram { value: test_val, cfg_dir: ".hop".to_string() };
      match program.jump_target(Link::new("myOtherLink")) {
        Ok(_) => assert_eq!(&vec!["/my/path/to/Otherlink".to_string()], &output),
        Err(e) => panic!("{}: Expected an Ok but got err", e)
      }
    }

    #[test]
    fn jump_target_not_found() {
      let mut output = vec![];
      let read_links =
        vec![
          LinkPair::new("myLink", "/my/path/to/link"),
          LinkPair::new("myOtherLink", "/my/path/to/Otherlink")
        ];

      let test_val = Test { out: &mut output, get_hop_home: None, read_dir_links: Ok(read_links), dir_exists: true, link_exists: false, write_link: None };
      let mut program = HopProgram { value: test_val, cfg_dir: ".hop".to_string() };
      match program.jump_target(Link::new("bizarre")) {
        Ok(_) => assert_eq!(&vec!["Could not find link: bizarre".to_string()], &output),
        Err(e) => panic!("{}: Expected an Ok but got err", e)
      }
    }

    #[test]
    fn jump_target_without_links() {
      let mut output = vec![];
      let read_links = vec![];

      let test_val = Test { out: &mut output, get_hop_home: None, read_dir_links: Ok(read_links), dir_exists: true, link_exists: false, write_link: None };
      let mut program = HopProgram { value: test_val, cfg_dir: ".hop".to_string() };
      match program.jump_target(Link::new("myLink")) {
        Ok(_) => assert_eq!(&vec!["Could not find link: myLink".to_string()], &output),
        Err(e) => panic!("{}: Expected an Ok but got err", e)
      }
    }

    #[test]
    fn mark_dir_success() {
      let mut output = vec![];
      let read_links = vec![];

      let test_val =
        Test {
          out: &mut output,
          get_hop_home: None,
          read_dir_links: Ok(read_links),
          dir_exists: true,
          link_exists: false,
          write_link: None
        };
      let mut program = HopProgram { value: test_val, cfg_dir: ".hop".to_string() };
      match program.mark_dir(LinkPair::new("myLink", "/my/path/to/link")) {
        Ok(_) => assert_eq!(&Vec::<String>::new(), &output),
        Err(e) => panic!("{}: Expected an Ok but got err", e)
      }
    }

    #[test]
    fn mark_dir_dir_does_not_exist() {
      let mut output = vec![];
      let read_links = vec![];

      let test_val =
        Test {
          out: &mut output,
          get_hop_home: None,
          read_dir_links: Ok(read_links),
          dir_exists: false,
          link_exists: false,
          write_link: None
        };
      let mut program = HopProgram { value: test_val, cfg_dir: ".hop".to_string() };
      match program.mark_dir(LinkPair::new("myLink", "/my/path/to/link")) {
        Ok(_) => panic!("Expected an Err but got Ok"),
        Err(e) => assert_eq!("A directory named `/my/path/to/link` does not exist or you do not have permission to it.", e.to_string()),
      }
    }

    #[test]
    fn mark_dir_link_exists() {
      let mut output = vec![];
      let read_links = vec![];

      let test_val =
        Test {
          out: &mut output,
          get_hop_home: None,
          read_dir_links: Ok(read_links),
          dir_exists: true,
          link_exists: true,
          write_link: None
        };
      let mut program = HopProgram { value: test_val, cfg_dir: ".hop".to_string() };
      match program.mark_dir(LinkPair::new("myLink", "/my/path/to/link")) {
        Ok(_) => panic!("Expected an Err but got Ok"),
        Err(e) => assert_eq!("A link named `myLink` already exists. Aborting mark creation.", e.to_string()),
      }
    }

    #[test]
    fn mark_dir_write_link_failed() {
      let mut output = vec![];
      let read_links = vec![];

      let test_val =
        Test {
          out: &mut output,
          get_hop_home: None,
          read_dir_links: Ok(read_links),
          dir_exists: true,
          link_exists: false,
          write_link: Some("Could not create link because this is a test".to_string())
        };
      let mut program = HopProgram { value: test_val, cfg_dir: ".hop".to_string() };
      match program.mark_dir(LinkPair::new("myLink", "/my/path/to/link")) {
        Ok(_) => panic!("Expected an Err but got Ok"),
        Err(e) => assert_eq!("Could not create link because this is a test", e.to_string()),
      }
    }
}
