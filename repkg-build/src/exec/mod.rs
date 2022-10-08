use color_eyre::Result;
use repkg_common::{Command, Rule};

use crate::Project;

pub mod executor;

pub use executor::*;

pub trait ExecutorT<'a> {
    fn execute(&self, rules: &Vec<&Rule>, project: &'a Project) -> Result<()> {
        for rule in rules {
            for command in &rule.cmds {
                self.run_command(command, project)?;
            }
        }

        Ok(())
    }

    fn run_command(&self, command: &Command, project: &'a Project) -> Result<()>;
}

pub trait CommandT {
    fn call(&self, args: &[&str]) -> Result<()>;
}

pub mod commands {
    use std::collections::HashMap;

    use color_eyre::Result;

    use super::CommandT;

    pub fn commands<'a>() -> HashMap<String, Box<dyn CommandT>> {
        let mut hashmap: HashMap<String, Box<dyn CommandT>> = HashMap::new();

        let commands: [Box<dyn CommandT>; 2] = [Box::new(EchoCommand), Box::new(SayCommand)];
        let names = vec!["echo", "say"];

        for (name, command) in names.into_iter().zip(commands) {
            hashmap.insert(name.to_string(), command);
        }

        hashmap
    }

    pub struct SayCommand;
    impl CommandT for SayCommand {
        fn call(&self, args: &[&str]) -> Result<()> {
            let to_print = args.join(" ");
            println!("say: {}", to_print);
            Ok(())
        }
    }

    pub struct EchoCommand;

    impl CommandT for EchoCommand {
        fn call(&self, args: &[&str]) -> Result<()> {
            let to_print = args.join(" ");
            println!("{}", to_print);
            Ok(())
        }
    }
}
