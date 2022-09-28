use std::{collections::HashMap, process};

use color_eyre::{eyre::eyre, Result};

use super::{commands, CmdProviderT, CommandT};

pub struct SystemCmdProvider {
    commands: HashMap<String, Box<dyn CommandT>>,
}

impl CmdProviderT<()> for SystemCmdProvider {
    fn cmd_inner(&self, program: &str, args: &[&str]) -> Result<()> {
        self.commands
            .get(program)
            .ok_or(eyre!("The command '{}' does not exist", program))?
            .call(args)
    }

    fn cmd_external(&self, program: &str, args: &[&str]) -> Result<()> {
        let status = process::Command::new(program).args(args).status()?;

        if !status.success() {
            return Err(eyre!(
                "Command '{}' failed with exit code '{}'",
                program,
                status.code().unwrap()
            ));
        }
        Ok(())
    }

    fn new() -> Self {
        Self {
            commands: commands::commands(),
        }
    }

    fn serve(&mut self, name: String, cmd: impl CommandT + 'static) {
        self.commands.insert(name, Box::new(cmd));
    }
}

impl SystemCmdProvider {}
