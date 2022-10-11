use color_eyre::Result;

use crate::sandbox::FileSystem;

pub mod executor;

pub use executor::*;

pub trait CommandT<F: FileSystem> {
    fn call(&self, sandbox: &mut F, args: &[&str]) -> Result<()>;
}

pub mod commands {
    use std::collections::HashMap;

    use color_eyre::Result;

    use crate::sandbox::FileSystem;

    use super::CommandT;

    pub fn commands<F: FileSystem>() -> HashMap<String, Box<dyn CommandT<F>>> {
        let mut hashmap: HashMap<String, Box<dyn CommandT<F>>> = HashMap::new();

        let commands: [Box<dyn CommandT<F>>; 2] = [Box::new(EchoCommand), Box::new(SayCommand)];
        let names = vec!["echo", "say"];

        for (name, command) in names.into_iter().zip(commands) {
            hashmap.insert(name.to_string(), command);
        }

        hashmap
    }

    pub struct SayCommand;
    impl<F: FileSystem> CommandT<F> for SayCommand {
        fn call(&self, _sandbox: &mut F, args: &[&str]) -> Result<()> {
            let to_print = args.join(" ");
            println!("say: {}", to_print);
            Ok(())
        }
    }

    pub struct EchoCommand;
    impl<F: FileSystem> CommandT<F> for EchoCommand {
        fn call(&self, _sandbox: &mut F, args: &[&str]) -> Result<()> {
            let to_print = args.join(" ");
            println!("{}", to_print);
            Ok(())
        }
    }
}
