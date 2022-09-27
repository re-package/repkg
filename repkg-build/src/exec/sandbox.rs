use std::collections::HashMap;

use color_eyre::{eyre::eyre, Result};

use super::{commands, CmdProviderT, CommandT};

pub struct SandBoxCmdProvider {
    commands: HashMap<String, Box<dyn CommandT>>,
}

impl CmdProviderT<()> for SandBoxCmdProvider {
    fn cmd_inner(&self, program: &str, args: &[&str]) -> Result<()> {
        self.commands
            .get(&program.to_string())
            .ok_or(eyre!("This command does not exist"))?
            .call(args)
    }

    fn cmd_external(&self, _program: &str, _args: &[&str]) -> Result<()> {
        Err(eyre!("Not allowed in sandbox mode"))
    }
}

impl SandBoxCmdProvider {
    pub fn new() -> Self {
        Self {
            commands: commands::commands(),
        }
    }
}
