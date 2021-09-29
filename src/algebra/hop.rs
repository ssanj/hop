use crate::models::HopEffect;

use super::{dirs::Dirs, std_io::StdIO, symlinks::SymLinks};

pub struct HopProgram<T>{
    pub value: T,
    pub cfg_dir: String
  }

impl <T> HopProgram<T>
  where
    T : Dirs + StdIO + SymLinks
  {

  pub fn list_links(&mut self) -> HopEffect<()> {
        let hop_home_dir = self.value.get_hop_home(&self.cfg_dir)?;
        let entries = self.value.read_dir_links(&hop_home_dir)?;

        for lp in entries {
            self.value.println(&format!("{}", lp.link))
        }

        Ok(())
  }
}