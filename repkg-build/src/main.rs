use std::{collections::BTreeMap, fs, rc::Rc};

use miette::Result;
use repkg_build::script::{ast::ASTBuilder, vm::VM, Value, ValueType};
use tree_sitter::Parser;

fn main() -> Result<()> {
    // let cli = Cli::parse();
    // repkg_build::run(cli)

    let mut parser = Parser::new();
    parser.set_language(tree_sitter_repkg::language()).unwrap();
    let text = fs::read_to_string("build.repkg").unwrap();
    let tree = parser.parse(&text, None).unwrap();
    let text = Rc::new(text);
    let name = Rc::new("build.repkg".to_string());
    let out = ASTBuilder::new(text.clone(), name).construct(tree)?;
    let mut vm = VM::init();
    let ffi_name = Rc::new("ffi".to_string());
    vm.with_ffi(Value::new(
        0..0,
        ValueType::Object(BTreeMap::from([(
            "bob".to_string(),
            Value::new(
                0..0,
                ValueType::String("BOB".to_string()),
                text.clone(),
                ffi_name.clone(),
            ),
        )])),
        text.clone(),
        ffi_name.clone(),
    ))?;
    vm.build(&out)?;

    Ok(())
}
