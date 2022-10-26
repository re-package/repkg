pub mod executor;
pub use executor::*;

pub mod tree_walker;

use miette::Result;

pub trait CommandT {
    fn call(&self, args: &[&str]) -> Result<()>;
}

pub mod commands {
    use std::collections::HashMap;

    use miette::Result;

    use super::CommandT;

    pub fn commands() -> HashMap<String, Box<dyn CommandT>> {
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
