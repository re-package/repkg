use miette::{Diagnostic, Result};

use repkg_core::{Project, Task};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("Attempted to get tasks for rule '{}' but it does not exist", .0)]
    #[diagnostic(code(repkg_build::task_order::RuleDoesntExist))]
    RuleDoesntExist(String),
}

pub fn calc_task_order<'a>(
    tasks: &'a [impl ToString],
    context: &'a Project,
) -> Result<Vec<&'a Task>> {
    let mut task_order = vec![];
    for task in tasks {
        let mut new_tasks = get_tasks(task, context)?;
        task_order.append(&mut new_tasks);
    }

    Ok(task_order)
}

fn get_tasks<'a>(initial: &'a impl ToString, project: &'a Project) -> Result<Vec<&'a Task>> {
    let mut exec_before = vec![];

    let initial = initial.to_string();
    let initial_rule = project
        .rules
        .get(&initial)
        .ok_or_else(|| Error::RuleDoesntExist(initial.clone()))?;

    exec_before.push(initial_rule);

    let gen_pre = |name: &String| -> String {
        let (first, rest) = name.split_at(1);
        format!("pre{}{}", first.to_uppercase(), rest)
    };
    let gen_dep = |name: &String| -> String { format!("{}Dependencies", name) };

    // Does the current rule have a pre{rule} rule
    let mut has_pre = project.rules.get(&gen_pre(&initial));
    let mut has_dep = project.rules.get(&gen_dep(&initial));
    loop {
        if initial.ends_with("Dependencies") {
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
    Ok(exec_before)
}
