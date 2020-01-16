use std::convert::From;
use std::path::Path;
use std::process::{Command, Stdio};
use std::vec::Vec;

use failure::Error;

use crate::config::app::PackageConfig;

pub trait PackageCommands {
    fn execute(&self, cwd: &Path) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct CommandList {
    cmds: Vec<String>,
}

impl PackageCommands for CommandList {
    fn execute(&self, cwd: &Path) -> Result<(), Error> {
        println!("Executing packaging commands...");
        for cmd in &self.cmds {
            println!("Running `{}`...", cmd);
            Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .stderr(Stdio::null())
                .stdout(Stdio::null())
                .current_dir(cwd)
                .status()
                .map_err(|e| format_err!("Cannot start command <{}>: ", e))
                .and_then(|status| {
                    if status.success() {
                        Ok(())
                    } else {
                        match status.code() {
                            Some(code) => Err(format_err!(
                                "Command <{}> returned exit status {:?}",
                                cmd,
                                code
                            )),
                            None => Err(format_err!("Command <{}> was aborted by a signal", cmd)),
                        }
                    }
                })?;
        }
        println!("Executed all packaging commands.");
        Ok(())
    }
}

impl<'a> From<&'a PackageConfig> for CommandList {
    fn from(config: &'a PackageConfig) -> Self {
        CommandList {
            cmds: config.before_cmds().clone(),
        }
    }
}

impl From<PackageConfig> for CommandList {
    fn from(config: PackageConfig) -> Self {
        CommandList {
            cmds: config.before_cmds().clone(),
        }
    }
}
