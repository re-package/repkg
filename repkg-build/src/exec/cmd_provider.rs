use color_eyre::{Result, eyre::eyre};

pub trait CmdProviderT<T> {
    fn cmd_inner(&self, program: &str, args: &[&str]) -> Result<T>;
    fn cmd_external(&self, program: &str, args: &[&str]) -> Result<T>;
}

pub struct NoneCmdProvider;

impl CmdProviderT<()> for NoneCmdProvider {
    fn cmd_inner(&self, _program: &str, _args: &[&str]) -> Result<()> {
        Err(eyre!("No cmd provider"))
    }

    fn cmd_external(&self, _program: &str, _args: &[&str]) -> Result<()> {
        Err(eyre!("No cmd provider"))
    }
}
