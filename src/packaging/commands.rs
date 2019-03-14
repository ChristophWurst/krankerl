use std::convert::From;
use std::path::Path;
use std::process::{Command, Stdio};
use std::vec::Vec;

use config::app::PackageConfig;

use failure::Error;
use indicatif::ProgressBar;

pub trait PackageCommands {
    fn execute(&self, cwd: &Path, progress: Option<&ProgressBar>) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct CommandList {
    cmds: Vec<String>,
}

impl PackageCommands for CommandList {
    fn execute(&self, cwd: &Path, progress: Option<&ProgressBar>) -> Result<(), Error> {
        progress.map(|prog| prog.set_message("Executing packaging commands..."));
        for cmd in &self.cmds {
            progress.map(|prog| prog.set_message(&format!("Running `{}`...", cmd)));
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
        progress.map(|prog| prog.finish_with_message("Executed all packaging commands."));
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
