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
