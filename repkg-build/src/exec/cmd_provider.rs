use color_eyre::{eyre::eyre, Result};

use super::CommandT;

pub trait CmdProviderT<T> {
    fn cmd_inner(&self, program: &str, args: &[&str]) -> Result<T>;
    fn cmd_external(&self, program: &str, args: &[&str]) -> Result<T>;
    fn new() -> Self;
    fn serve(&mut self, name: String, cmd: impl CommandT + 'static);
}

pub struct NoneCmdProvider;

impl CmdProviderT<()> for NoneCmdProvider {
    fn cmd_inner(&self, _program: &str, _args: &[&str]) -> Result<()> {
        Err(eyre!("No cmd provider"))
    }

    fn cmd_external(&self, _program: &str, _args: &[&str]) -> Result<()> {
        Err(eyre!("No cmd provider"))
    }

    fn new() -> Self {
        Self {}
    }

    fn serve(&mut self, _: String, _: impl CommandT + 'static) {
        unimplemented!()
    }
}
