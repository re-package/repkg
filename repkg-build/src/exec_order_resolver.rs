use crate::{rule::Rule, Name, Program};

pub trait Resolver {
    fn get_tasks<'a>(initial: &'a Rule, program: &'a Program) -> Vec<&'a Rule>;
}

pub struct Resolver1;

impl Resolver for Resolver1 {
    fn get_tasks<'a>(initial: &'a Rule, program: &'a Program) -> Vec<&'a Rule> {
        Self::exec_before(initial, program)
    }
}

impl Resolver1 {
    fn exec_before<'a>(rule: &'a Rule, program: &'a Program) -> Vec<&'a Rule> {
        let mut exec_before = vec![];

        exec_before.push(rule);

        let gen_pre = |name: &Name| -> Name {
            let (first, rest) = name.0.split_at(1);
            Name(format!("pre{}{}", first.to_uppercase(), rest))
        };
        let gen_dep = |name: &Name| -> Name { Name(format!("{}Dependencies", name.0)) };

        // Does the current rule have a pre{rule} rule
        let mut has_pre = program.rules.get(&gen_pre(&rule.name));
        let mut has_dep = program.rules.get(&gen_dep(&rule.name));
        loop {
            if let Some(dep) = has_dep {
                exec_before.push(dep);
            }
            if let Some(pre) = has_pre {
                has_pre = program.rules.get(&gen_pre(&pre.name));
                has_dep = program.rules.get(&gen_dep(&pre.name));
                exec_before.push(pre);
            } else {
                break;
            }
        }

        exec_before.reverse();
        exec_before
    }
}
