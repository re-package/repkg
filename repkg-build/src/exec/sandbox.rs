use std::collections::HashMap;

use color_eyre::{eyre::eyre, Result};

use super::{commands, CmdProviderT, CommandT};

pub struct SandboxCmdProvider {
    commands: HashMap<String, Box<dyn CommandT>>,
}

impl CmdProviderT<()> for SandboxCmdProvider {
    fn cmd_inner(&self, program: &str, args: &[&str]) -> Result<()> {
        self.commands
            .get(&program.to_string())
            .ok_or(eyre!("The command '{}' does not exist", program))?
            .call(args)
    }

    fn cmd_external(&self, _program: &str, _args: &[&str]) -> Result<()> {
        Err(eyre!("Not allowed in sandbox mode"))
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
