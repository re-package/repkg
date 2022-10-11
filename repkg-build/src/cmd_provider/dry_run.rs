use std::collections::HashMap;

use crate::exec::{commands, CommandT};

use super::CmdProvider;

use color_eyre::eyre::eyre;

pub struct DryRunCMD {
    commands: HashMap<String, Box<dyn CommandT>>,
}

impl<'a> CmdProvider<'a> for DryRunCMD {
    fn new() -> Self {
        Self {
            commands: commands::commands(),
        }
    }

    fn command(&mut self, program: &str, args: &[&str]) -> color_eyre::Result<()> {
        self.commands
            .get(&program.to_string())
            .ok_or(eyre!("The command '{}' does not exist", program))?
            .call(args)
    }

    fn reg_cmd(&mut self, program: &str, cmd: impl CommandT + 'static) {
        self.commands.insert(program.to_string(), Box::new(cmd));
    }
}

impl<'a> Default for DryRunCMD {
    fn default() -> Self {
        Self::new()
    }
}
