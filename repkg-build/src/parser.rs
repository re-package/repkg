use std::{collections::BTreeMap, fs, path::PathBuf};

use pom::{
    char_class::{self, *},
    parser::*,
};

use miette::{Diagnostic, Result};
use thiserror::Error;

use crate::ASTNode;

use repkg_common::{Command, Name, Project, Rule};

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(code(repkg_build::parser::parse_error))]
    ParseError(#[from] pom::Error),
}

pub fn body<'a>() -> Parser<'a, u8, Result<Project>> {
    spaced_newline(
        call(project).map(|x| Result::<ASTNode>::Ok(ASTNode::Project(x?)))
            | rule().map(|x| Ok(ASTNode::Rule(x))),
    )
    .repeat(0..)
    .map(|nodes| {
        let mut projects = BTreeMap::new();
        let mut imports = vec![];
        let mut rules = BTreeMap::new();
        for node in nodes {
            match node? {
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
        Ok(Project {
            projects,
            rules,
            name: "root".into(),
            in_: PathBuf::from("."),
        })
    })
}

pub fn parser<'a>() -> Parser<'a, u8, Result<Project>> {
    body() - end()
}

pub fn project<'a>() -> Parser<'a, u8, Result<Project>> {
    (seq(b"project") * spaced(id())
        + spaced(seq(b"in") * spaced(string())).opt()
        + spaced(seq(b"at") * spaced(string())).opt()
        + ((sym(b'{') * call(body) - sym(b'}'))
            | sym(b';').map(|_| {
                Ok(Project {
                    name: "".into(),
                    projects: BTreeMap::new(),
                    rules: BTreeMap::new(),
                    in_: PathBuf::from("."),
                })
            })))
    .map(|(((name, in_), at_), body)| {
        let mut body = body?;
        if let Some(at) = at_ {
            let content = fs::read_to_string(at).map_err(crate::io_error)?;
            let mut project = parser()
                .parse(content.as_bytes())
                .map_err(|e| Error::ParseError(e))??;

            body.projects.append(&mut project.projects);
            body.rules.append(&mut project.rules);
        }
        if let Some(in_) = &in_ {
            let at = PathBuf::from(in_).join(".repkg");
            if at.exists() {
                let content = fs::read_to_string(at).map_err(crate::io_error)?;
                let mut project = parser()
                    .parse(content.as_bytes())
                    .map_err(|e| Error::ParseError(e))??;

                body.projects.append(&mut project.projects);
                body.rules.append(&mut project.rules);
            }
        }
        Ok(Project {
            name,
            projects: body.projects,
            rules: body.rules,
            in_: in_.map(|x| PathBuf::from(x)).unwrap_or(PathBuf::from(".")),
        })
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
        + (is_a(alphanum) | sym(b'-') | sym(b'/') | sym(b'_')).repeat(0..))
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
            build : $echo blah
        }";
        let project = super::project().parse(program).unwrap().unwrap();
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
        let project = super::project().parse(program).unwrap().unwrap();

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

        let result = super::parser().parse(program).unwrap().unwrap();

        dbg!(&result);

        assert!(result.projects.len() == 2);
        assert!(result.rules.len() == 3);
    }
}
