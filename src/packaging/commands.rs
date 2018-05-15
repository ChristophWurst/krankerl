use std::convert::Into;
use std::path::Path;
use std::process::{Command, Stdio};
use std::vec::Vec;

use config::app::PackageConfig;

use failure::Error;
use indicatif::ProgressBar;

pub trait PackageCommands {
    fn execute(&self, cwd: &Path, progress: ProgressBar) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct CommandList {
    cmds: Vec<String>,
}

impl PackageCommands for CommandList {
    fn execute(&self, cwd: &Path, progress: ProgressBar) -> Result<(), Error> {
        progress.set_message("Executing packaging commands...");
        for cmd in &self.cmds {
            progress.set_message(&format!("Running `{}`...", cmd));
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
        progress.finish_with_message("Executed all packaging commands.");
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
