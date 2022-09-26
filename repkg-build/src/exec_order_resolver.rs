use repkg_common::Rule;

use crate::{Name, Project};

pub trait ResolverT {
    fn get_tasks<'a>(initial: &'a Rule, project: &'a Project) -> Vec<&'a Rule>;
}

pub struct Resolver;

impl ResolverT for Resolver {
    fn get_tasks<'a>(initial: &'a Rule, project: &'a Project) -> Vec<&'a Rule> {
        let mut exec_before = vec![];

        exec_before.push(initial);

        let gen_pre = |name: &Name| -> Name {
            let (first, rest) = name.0.split_at(1);
            Name(format!("pre{}{}", first.to_uppercase(), rest))
        };
        let gen_dep = |name: &Name| -> Name { Name(format!("{}Dependencies", name.0)) };

        // Does the current rule have a pre{rule} rule
        let mut has_pre = project.rules.get(&gen_pre(&initial.name));
        let mut has_dep = project.rules.get(&gen_dep(&initial.name));
        loop {
            if initial.name.0.ends_with("Dependencies") {
                break;
            }
            if let Some(dep) = has_dep {
                exec_before.push(dep);
            }
            if let Some(pre) = has_pre {
                has_pre = project.rules.get(&gen_pre(&pre.name));
                has_dep = project.rules.get(&gen_dep(&pre.name));
                exec_before.push(pre);
            } else {
                break;
            }
        }

        exec_before.reverse();
        exec_before
    }
}
