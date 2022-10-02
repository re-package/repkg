use std::{collections::BTreeMap, path::PathBuf};

use pom::{
    char_class::{self, *},
    parser::*,
};

use crate::ASTNode;

use repkg_common::{Command, Name, Project, Rule};

pub fn parser<'a>() -> Parser<'a, u8, Project> {
    spaced_newline(project().map(|x| ASTNode::Project(x)) | rule().map(|x| ASTNode::Rule(x)))
        .repeat(0..)
        .map(|nodes| {
            let mut projects = BTreeMap::new();
            let mut imports = vec![];
            let mut rules = BTreeMap::new();
            for node in nodes {
                match node {
                    ASTNode::Project(proj) => {
                        projects.insert(proj.name.to_owned(), proj);
                    }
                    ASTNode::Import(import) => {
                        imports.push(import);
                    }
                    ASTNode::Rule(rule) => {
                        rules.insert(rule.name.to_owned(), rule);
                    }
                }
            }
            Project {
                projects,
                rules,
                name: "root".into(),
                in_: PathBuf::from("."),
                at_: None,
            }
        })
}

pub fn project<'a>() -> Parser<'a, u8, Project> {
    (seq(b"project") * spaced(id())
        + spaced(seq(b"in") * spaced(string())).opt()
        + spaced(seq(b"at") * spaced(string())).opt()
        + ((sym(b'{')
            * space_newline()
            * (space_newline() * rule().map(|x| (x.name.to_owned(), x)) - space_newline())
                .repeat(0..)
            - space()
            - sym(b'}'))
            | spaced(sym(b';')).map(|_| Vec::new())))
    .map(|(((name, in_), at), rules)| Project {
        name,
        projects: BTreeMap::new(),
        rules: BTreeMap::from_iter(rules.into_iter()),
        in_: PathBuf::from(in_.unwrap_or(".".to_string())),
        at_: at.map(|x| PathBuf::from(x)),
    })
}

fn rule<'a>() -> Parser<'a, u8, Rule> {
    (spaced(id())
        + ((sym(b'{') * spaced_newline(command()).repeat(1..) - space() - sym(b'}'))
            | (space() * sym(b':') * spaced(command()).map(|x| vec![x]))))
    .map(|(name, cmds)| Rule { name, cmds })
}

pub fn command<'a>() -> Parser<'a, u8, Command> {
    (spaced((sym(b'#') | sym(b'$') | sym(b'!')).opt() + (sym(b'.').opt() * id()).repeat(1..))
        + spaced(string().map(|x| format!("{}", x)) | id().map(|x| x.0)).repeat(0..))
    .map(|((prefix, programs), args)| Command {
        prefix: prefix.map(|x| char::from_u32(x as u32).unwrap()),
        programs: programs.into_iter().map(|x| x.0).collect(),
        args,
    })
}

fn string<'a>() -> Parser<'a, u8, String> {
    (sym(b'"') * none_of(b"\"").repeat(0..) - sym(b'"'))
        .map(|bytes| String::from_utf8(bytes).unwrap())
}

fn id<'a>() -> Parser<'a, u8, Name> {
    ((is_a(alpha) | sym(b'-') | sym(b'/') | sym(b'$') | sym(b'!') | sym(b'#'))
        + (is_a(alphanum) | sym(b'-') | sym(b'-')).repeat(0..))
    .map(|(first, rest)| {
        Name(format!(
            "{}{}",
            first as char,
            String::from_utf8(rest).unwrap()
        ))
    })
}

fn spaced<'a, T: 'a>(parser: Parser<'a, u8, T>) -> Parser<'a, u8, T> {
    space() * parser - space()
}

fn spaced_newline<'a, T: 'a>(parser: Parser<'a, u8, T>) -> Parser<'a, u8, T> {
    space_newline() * parser - space_newline()
}

fn space_newline<'a>() -> Parser<'a, u8, ()> {
    is_a(multispace).repeat(0..).discard()
}

fn space<'a>() -> Parser<'a, u8, ()> {
    is_a(char_class::space).repeat(0..).discard()
}

#[cfg(test)]
mod tests {
    use repkg_common::Command;

    use std::path::PathBuf;

    #[test]
    fn command() {
        let program = b"cargo test \"build bob\" run --release";
        let command = super::command().parse(program).unwrap();

        dbg!(&command);

        assert!(command.programs == vec!["cargo"]);
        assert!(command.args == vec!["test", "build bob", "run", "--release"]);
        assert!(command.prefix == None);
    }

    #[test]
    fn rule() {
        let program = b"build {
            cargo build
            other build
        }";
        let rule = super::rule().parse(program).unwrap();

        assert!(rule.name == "build".into());
        assert!(rule.cmds.len() == 2);
    }

    #[test]
    fn short_rule() {
        let program = b"build : cargo build";
        let rule = super::rule().parse(program).unwrap();

        assert!(rule.name == "build".into());
        assert!(
            rule.cmds
                == vec![Command {
                    programs: vec!["cargo".to_string()],
                    args: vec!["build".to_string()],
                    prefix: None,
                }]
        );
    }

    #[test]
    fn project() {
        let program = b"project my-project in \"my-project\" {
        }";
        let project = super::project().parse(program).unwrap();
        assert!(project.name == "my-project".into());
        assert!(project.in_ == PathBuf::from("my-project"));
    }

    #[test]
    fn project_rules() {
        let program = b"project my_project {
            build {
                cargo build
            }

            test {
                cargo nextest run
            }
        }";
        let project = super::project().parse(program).unwrap();

        assert!(project.rules.len() == 2);

        assert!(project
            .rules
            .iter()
            .any(|(name, _rule)| name == &"build".into()));
        assert!(project
            .rules
            .iter()
            .any(|(name, _rule)| name == &"test".into()));
    }

    #[test]
    fn many_projects() {
        let program = b"project my_project {
            build {
                cargo build
            }

            test {
                cargo nextest run
            }
        }
        
        project other_my_project {
            build {
                cargo build
            }
            
            test {
                cargo test
            }
        }
        
        build : cargo build
        
        test {
            cargo nextest run
        }
        
        buildDependencies {
            #rust.stable.cargo
        }";

        let result = super::parser().parse(program).unwrap();

        dbg!(&result);

        assert!(result.projects.len() == 2);
        assert!(result.rules.len() == 3);
    }
}
