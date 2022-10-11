use color_eyre::Result;

use crate::exec::CommandT;

pub mod dry_run;
pub mod sandbox;

pub trait CmdProvider<'a> {
    fn command(&mut self, program: &str, args: &[&str]) -> Result<()>;
    fn reg_cmd(&mut self, program: &str, cmd: impl CommandT + 'static)
    where
        Self: Sized;
    fn new() -> Self;
}
