use crate::models::{HopEffect, Link, LinkPair, HomeType};
use crate::program::io_error;

use super::{
    directories::Directories, std_io::StdIO, symlinks::SymLink, symlinks::SymLinks,
    user_dirs::UserDirs,
};

/// The data required to run hop
pub struct HopProgram<T> {
    pub value: T,
    pub hop_home_dir: HomeType,
}

#[derive(Debug, PartialEq)]
pub enum DeleteStatus {
    DeleteAborted,
    DeleteSucceeded(LinkPair),
}

impl<T> HopProgram<T>
where
    T: UserDirs + StdIO + SymLinks + Directories,
{

    pub fn list_links(&self) -> HopEffect<Vec<LinkPair>> {
        self.get_link_pairs()
    }

    pub fn tabulate_links(&self) -> HopEffect<Vec<LinkPair>> {
        self.get_link_pairs()
    }

    pub fn jump_target(&self, link: Link) -> HopEffect<String> {
        let entries = self.get_link_pairs()?;
        match entries.iter().find(|&lp| lp.link == link) {
            Some(found_lp) => Ok(format!("{}", found_lp.target)),
            None => Err(io_error(&format!("Could not find link: {}", link))),
        }
    }

    fn get_link_pairs(&self) -> HopEffect<Vec<LinkPair>> {
        let hop_home_dir = self.value.get_hop_home(&self.hop_home_dir)?;
        let entries = self.value.read_dir_links(&hop_home_dir)?;

        Ok(entries.to_vec())
    }

    pub fn mark_dir(&self, pair: &LinkPair) -> HopEffect<String> {
        let hop_home = self.value.get_hop_home(&self.hop_home_dir)?;
        let symlink_path = (hop_home).join(&pair.link);

        let target_path = pair.target.to_path_buf();

        let resolved_target_path =
          // Check if the path is relative and add the current directory to it
          if target_path.is_relative() {
            let current_dir = std::env::current_dir()?;
            if pair.target.as_ref() == std::path::Path::new(".") {
              // If the user entered ".", then just use the current directory
              current_dir
            } else {
              // Otherwise append the current directory to the relative path
              current_dir.join(&target_path)
            }
          } else {
            target_path
          };

        //TODO: Send in a LinkTarget
        if self.value.dir_exists(&resolved_target_path)? {
            //TODO: Send in a SymLink
            if self.value.link_exists(&symlink_path)? {
                Err(io_error(&format!(
                    "A link named `{}` already exists. Aborting mark creation.",
                    pair.link
                )))
            } else {
                self.value
                    .write_link(&SymLink(symlink_path), &resolved_target_path)
                    .map(|_| resolved_target_path.to_string_lossy().to_string())
            }
        } else {
            Err(io_error(&format!(
                "A directory named `{}` does not exist or you do not have permission to it.",
                &pair.target
            )))
        }
    }

    pub fn delete_link(&self, link: &Link) -> HopEffect<DeleteStatus> {
        let link_pairs = self.get_link_pairs()?;

        match link_pairs.iter().find(|lp| &lp.link == link) {
            Some(pair) => {
                let prompt_message = format!(
                    "Are you sure you want to delete {} which links to {} ?",
                    pair.link, pair.target
                );

                let no_action = || Ok(DeleteStatus::DeleteAborted);

                let yes_action = || {
                    let hop_home = &self.value.get_hop_home(&self.hop_home_dir)?;
                    self.value.delete_link(hop_home, pair)?;

                    Ok(DeleteStatus::DeleteSucceeded(pair.clone()))
                };

                self.prompt_user(&prompt_message, yes_action, no_action)
            }

            None => Err(io_error(&format!(
                "Could not find link named:`{}` for deletion",
                link
            ))),
        }
    }

    fn prompt_user<Y, N, R>(&self, message: &str, yes_action: Y, no_action: N) -> HopEffect<R>
    where
        Y: FnOnce() -> HopEffect<R>,
        N: FnOnce() -> HopEffect<R>,
    {
        self.value.println(message);
        let buffer = self.value.readln()?;
        let response = buffer
            .lines()
            .next()
            .ok_or_else(|| io_error("Could not retrieve lines from stdio"))?;
        match response {
            "Y" | "y" => yes_action(),
            _ => no_action(),
        }
    }
}

#[cfg(test)]
mod tests;
