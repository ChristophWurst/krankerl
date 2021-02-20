use color_eyre::{eyre::WrapErr, Result};
use std::convert;
use std::ffi;
use std::process::Command;

pub struct Occ<P> {
    path: P,
}

impl<P> Occ<P>
where
    P: convert::AsRef<ffi::OsStr>,
{
    pub fn new(occ_path: P) -> Self {
        Occ { path: occ_path }
    }

    fn start_command(&self) -> Command {
        Command::new(&self.path)
    }

    fn invoke_command(&self, mut cmd: Command) -> Result<()> {
        cmd.status()?;
        Ok(())
    }

    pub fn enable_app(&self, app_id: &String) -> Result<()> {
        let mut cmd = self.start_command();
        cmd.arg("app:enable").arg(app_id);
        self.invoke_command(cmd)
            .wrap_err(format!("Failed to run `occ app:enable` command"))
    }

    pub fn disable_app(&self, app_id: &String) -> Result<()> {
        let mut cmd = self.start_command();
        cmd.arg("app:disable").arg(app_id);
        self.invoke_command(cmd)
            .wrap_err(format!("Failed to run `occ app:disable` command"))
    }
}
