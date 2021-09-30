use crate::models::HopEffect;

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
        let hop_home_dir = self.value.get_hop_home(&self.cfg_dir)?;
        let entries = self.value.read_dir_links(&hop_home_dir)?;

        entries.iter().for_each(|lp| self.value.println(&format!("{}", lp.link)));

        Ok(())
  }
}

#[cfg(test)]
mod tests {
    use crate::algebra::{std_io::StdIO, symlinks::SymLinks, user_dirs::UserDirs};
    use crate::models::{HopEffect, LinkPair};
    use super::HopProgram;

    use std::path::PathBuf;

    struct Test<'a> {
      out: &'a mut Vec<String>
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
        Ok(PathBuf::from("/xyz/.your-hop"))
      }
    }

    impl SymLinks for Test<'_> {
      fn read_dir_links(&self, dir_path: &PathBuf) -> HopEffect<Vec<LinkPair>> {
        Ok(
          vec![
            LinkPair::new("myLink", "/my/path/to/link"),
            LinkPair::new("myOtherLink", "/my/path/to/Otherlink")
          ])
      }
    }


    #[test]
    fn list_links_success() {
      let mut output = vec![];
      let test_val = Test { out: &mut output };
      let mut program = HopProgram { value: test_val, cfg_dir: ".hop".to_string() };
      if let Ok(_) = program.list_links() {
        assert_eq!(&vec!["myLink".to_string(), "myOtherLink".to_string()], &output)
      }
    }

}
