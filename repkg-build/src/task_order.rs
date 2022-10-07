use color_eyre::{eyre::eyre, Result};

use repkg_common::{Name, Project, Rule};

pub fn calc_task_order<'a>(tasks: &[&'a Name], context: &'a Project) -> Result<Vec<&'a Rule>> {
    let mut task_order = vec![];
    for task in tasks {
        let mut new_tasks = get_tasks(task, context)?;
        task_order.append(&mut new_tasks);
    }

    Ok(task_order)
}

fn get_tasks<'a>(initial: &'a Name, project: &'a Project) -> Result<Vec<&'a Rule>> {
    let mut exec_before = vec![];

    let initial_rule = project.rules.get(initial).ok_or(eyre!(
        "Attempted to get tasks for rule '{}' but it does not exist",
        initial.0
    ))?;

    exec_before.push(initial_rule);

    let gen_pre = |name: &Name| -> Name {
        let (first, rest) = name.0.split_at(1);
        Name(format!("pre{}{}", first.to_uppercase(), rest))
    };
    let gen_dep = |name: &Name| -> Name { Name(format!("{}Dependencies", name.0)) };

    // Does the current rule have a pre{rule} rule
    let mut has_pre = project.rules.get(&gen_pre(initial));
    let mut has_dep = project.rules.get(&gen_dep(initial));
    loop {
        if initial.0.ends_with("Dependencies") {
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
