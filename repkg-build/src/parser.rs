use std::collections::BTreeMap;

use pom::{char_class::*, parser::*};

use crate::{rule::Rule, ASTNode, FunctionCall, Import, Name, Program, Project, Value};

pub fn parser<'a>() -> Parser<'a, u8, Program> {
    (space() * project().map(|x| ASTNode::Project(x)))
        .repeat(0..)
        .map(|nodes| {
            let functions = BTreeMap::new();
            let mut projects = BTreeMap::new();
            let mut imports: Vec<Import> = vec![];
            for node in nodes {
                let _ = match node {
                    ASTNode::Project(proj) => {
                        projects.insert(proj.name.to_owned(), proj);
                    }
                    ASTNode::Import(import) => {
                        imports.push(import);
                    }
                };
            }
            Program {
                functions,
                projects,
                imports,
            }
        })
}

pub fn project<'a>() -> Parser<'a, u8, Project> {
    (seq(b"project") * space() * id() - space() - sym(b'{') - space()
        + (space() * rule() - space()).repeat(0..)
        - space()
        - sym(b'}'))
    .map(|(name, rules)| Project { name, rules })
}

fn space<'a>() -> Parser<'a, u8, ()> {
    one_of(b" \t\r\n").repeat(0..).discard()
}

fn spaced<'a, T: 'a>(parser: Parser<'a, u8, T>) -> Parser<'a, u8, T> {
    space() * parser - space()
}

fn statement<'a>() -> Parser<'a, u8, Value> {
    function_call().map(|x| Value::FunctionCall(x)) - sym(b';')
}

fn string<'a>() -> Parser<'a, u8, String> {
    (sym(b'"') * is_a(alphanum).repeat(0..) - sym(b'"'))
        .map(|bytes| String::from_utf8(bytes).unwrap())
}

fn function_call<'a>() -> Parser<'a, u8, FunctionCall> {
    (id() - sym(b' ').repeat(0..) - sym(b'(')
        + (space() * (string().map(|x| Value::String(x))) - space()).repeat(0..2)
        + (space() * sym(b',') * space() * (string().map(|x| Value::String(x)))
            - space()
            - sym(b','))
        .repeat(0..)
        - sym(b')'))
    .map(|((name, mut param1), mut params)| {
        param1.append(&mut params);
        FunctionCall {
            func_name: name,
            arguments: param1,
        }
    })
}

fn rule<'a>() -> Parser<'a, u8, Rule> {
    (seq(b"rule") * spaced(id()) - sym(b'{') + spaced(statement()).repeat(0..) - sym(b'}'))
        .map(|(name, values)| Rule { name, values })
}

fn id<'a>() -> Parser<'a, u8, Name> {
    (is_a(alpha) + (is_a(alphanum) | sym(b'_')).repeat(0..)).map(|(first, rest)| {
        Name(format!(
            "{}{}",
            first as char,
            String::from_utf8(rest).unwrap()
        ))
    })
}

#[cfg(test)]
mod tests {
    use crate::{FunctionCall, Value};

    #[test]
    fn rule() {
        let program = b"rule my_rule {
            function_call();

            other_function_call();
        }";
        let rule = super::rule().parse(program).unwrap();
        assert!(rule.name == "my_rule".into());

        assert!(rule.values.len() == 2);

        assert!(rule.values.contains(&Value::FunctionCall(FunctionCall {
            func_name: "function_call".into(),
            arguments: vec![],
        })));

        assert!(rule.values.contains(&Value::FunctionCall(FunctionCall {
            func_name: "other_function_call".into(),
            arguments: vec![],
        })));
    }

    #[test]
    fn function_call_with_params() {
        let program = b"function_call(\"test\")";
        let func_call = super::function_call().parse(program).unwrap();

        assert!(func_call.func_name == "function_call".into());
        assert!(func_call.arguments == vec![Value::String("test".to_string())]);
    }

    #[test]
    fn project() {
        let program = b"project my_project {
        }";
        let project = super::project().parse(program).unwrap();
        assert!(project.name == "my_project".into());
    }

    #[test]
    fn project_rules() {
        let program = b"project my_project {
            rule my_rule {
                function_call();
                other_function_call();



                another_function_call();
            }

            rule my_other_rule {
                just_call_one();
            }
        }";
        let project = super::project().parse(program).unwrap();

        assert!(project.rules.len() == 2);

        assert!(project
            .rules
            .iter()
            .any(|rule| rule.name == "my_rule".into()));
        assert!(project
            .rules
            .iter()
            .any(|rule| rule.name == "my_other_rule".into()));
    }

    #[test]
    fn many_projects() {
        let program = b"project my_project {
            rule build {
                function_call();
            }
        }
        
        project my_other_project {
            rule test {
                function_call();
            }
        }";

        let result = super::parser().parse(program).unwrap();

        assert!(result.projects.len() == 2);
    }
}
