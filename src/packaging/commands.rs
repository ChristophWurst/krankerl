use std::convert::From;
use std::path::Path;
use std::process::Command;
use std::vec::Vec;

use color_eyre::{Report, Result};

use crate::config::app::PackageConfig;
use color_eyre::eyre::WrapErr;

pub trait PackageCommands {
    fn execute(&self, cwd: &Path) -> Result<()>;
}

#[derive(Debug)]
pub struct CommandList {
    cmds: Vec<String>,
}

impl PackageCommands for CommandList {
    fn execute(&self, cwd: &Path) -> Result<()> {
        println!("Executing packaging commands...");
        for cmd in &self.cmds {
            println!("Running `{}`...", cmd);
            Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .current_dir(cwd)
                .output()
                .wrap_err_with(|| format!("Cannot start command <{}>: ", cmd))
                .and_then(|output| {
                    if output.status.success() {
                        Ok(())
                    } else {
                        match output.status.code() {
                            Some(code) => Err(Report::msg(format!(
                                "Command <{}> returned exit status {:?}\n\nstdout: {}\nstderr: {}",
                                cmd,
                                code,
                                String::from_utf8_lossy(&output.stdout),
                                String::from_utf8_lossy(&output.stderr)
                            ))),
                            None => Err(Report::msg(format!(
                                "Command <{}> was aborted by a signal",
                                cmd
                            ))),
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
