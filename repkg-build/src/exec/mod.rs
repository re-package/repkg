use color_eyre::Result;

use crate::sandbox::{FileSystem, SandboxT};

pub mod executor;

pub use executor::*;

pub trait CommandT<'a, F: FileSystem, S: SandboxT<'a, F>> {
    fn call(&self, sandbox: &S, args: &[&str]) -> Result<()>;
}

pub mod commands {
    use std::collections::HashMap;

    use color_eyre::Result;

    use crate::sandbox::{FileSystem, SandboxT};

    use super::CommandT;

    pub fn commands<'a, F: FileSystem, S: SandboxT<'a, F>>(
    ) -> HashMap<String, Box<dyn CommandT<'a, F, S>>> {
        let mut hashmap: HashMap<String, Box<dyn CommandT<F, S>>> = HashMap::new();

        let commands: [Box<dyn CommandT<F, S>>; 2] = [Box::new(EchoCommand), Box::new(SayCommand)];
        let names = vec!["echo", "say"];

        for (name, command) in names.into_iter().zip(commands) {
            hashmap.insert(name.to_string(), command);
        }

        hashmap
    }

    pub struct SayCommand;
    impl<'a, F: FileSystem, S: SandboxT<'a, F>> CommandT<'a, F, S> for SayCommand {
        fn call(&self, _sandbox: &S, args: &[&str]) -> Result<()> {
            let to_print = args.join(" ");
            println!("say: {}", to_print);
            Ok(())
        }
    }

    pub struct EchoCommand;
    impl<'a, F: FileSystem, S: SandboxT<'a, F>> CommandT<'a, F, S> for EchoCommand {
        fn call(&self, _sandbox: &S, args: &[&str]) -> Result<()> {
            let to_print = args.join(" ");
            println!("{}", to_print);
            Ok(())
        }
    }
}
