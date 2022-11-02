use std::{fs, path::PathBuf};

use clap::Parser;
use miette::{Diagnostic, IntoDiagnostic, Result};
use repkg_script::exec::tree_exec;
use thiserror::Error;

fn main() -> Result<()> {
    // parser_new::parser::Parser::new(&fs::read_to_string(".repkg").into_diagnostic()?).parse()?;

    // let walker = TreeWalker::parse(".repkg")?;
    // let context = walker.walk()?;

    // ContextExecutor::execute(
    //     &context,
    //     &Registry {},
    //     vec![],
    //     &repkg_script::script_std::make(),
    // )?;

    let mut cli = Cli::parse();

    run(&mut cli)?;

    Ok(())
}

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("No RePKG package file found")]
    #[diagnostic(code(repkg_build::no_package_file))]
    NoPackageFile,
    #[error("Project '{}' does not exist", .0)]
    #[diagnostic(code(repkg_build::project_doesnt_exist))]
    ProjectDoesntExist(String),
}

fn run(cli: &mut Cli) -> Result<()> {
    let mut parser = tree_sitter::Parser::new();
    let source = fs::read_to_string(&cli.file).into_diagnostic()?;
    parser
        .set_language(tree_sitter_repkg::language())
        .into_diagnostic()?;
    let tree = parser.parse(&source, None).unwrap();

    let executor = tree_exec::Executor::new(tree, source);
    executor.execute_tree()?;

    Ok(())

    // let parse_output = TreeWalker::parse(&cli.file)?.walk()?;

    // let standard_lib = script_std::make();
    // let registry = Registry {};
    // ContextExecutor::execute(&parse_output, &registry, vec![], &standard_lib)
}

#[derive(Parser)]
struct Cli {
    #[clap(short, long, default_value = ".repkg")]
    file: PathBuf,
}
