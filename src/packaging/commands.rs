use std::convert::Into;
use std::path::Path;
use std::process::Command;
use std::vec::Vec;

use config::app::PackageConfig;
use error;

pub trait PackageCommands {
    fn execute(&self, cwd: &Path) -> Result<(), error::Error>;
}

pub struct CommandList {
    cmds: Vec<String>,
}

impl PackageCommands for CommandList {
    fn execute(&self, cwd: &Path) -> Result<(), error::Error> {
        for cmd in &self.cmds {
            println!("Running {}", cmd);
            Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .current_dir(cwd)
                .status()
                .map_err(|e| {
                    error::Error::Other(format!("Cannot start command <{}>: ", e))
                })
                .and_then(|status| {
                    if status.success() {
                        Ok(())
                    } else {
                        match status.code() {
                            Some(code) => Err(error::Error::Other(
                                format!("Command <{}> returned exit status {:?}", cmd, code),
                            )),
                            None => Err(error::Error::Other(
                                format!("Command <{}> was aborted by a signal", cmd),
                            )),
                        }
                    }
                })?;
        }
        Ok(())
    }
}

impl<'a> Into<CommandList> for &'a PackageConfig {
    fn into(self) -> CommandList {
        CommandList {
            cmds: self.before_cmds().clone(),
        }
    }
}

impl Into<CommandList> for PackageConfig {
    fn into(self) -> CommandList {
        CommandList {
            cmds: self.before_cmds().clone(),
        }
    }
}
