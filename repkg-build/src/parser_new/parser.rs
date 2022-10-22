use super::token::Token;

use logos::{Lexer, Logos};
use miette::Result;

pub struct Parser<'a> {
    source: &'a str,
    lexer: Lexer<'a, Token>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            lexer: Token::lexer(source),
        }
    }

    pub fn parse(&mut self) -> Result<Project> {
        parse_impl::parse(&mut self.lexer, self.source)
    }
}

pub use parse_impl::Error;
use repkg_common::Project;

mod parse_impl {
    use std::{collections::BTreeMap, path::PathBuf};

    use crate::parser_new::token::{Token, TokenType};

    use logos::Lexer;
    use miette::{bail, Diagnostic, Result, SourceSpan};
    use repkg_common::{Command, Project, Task};
    use thiserror::Error;

    use self::Error::*;

    #[derive(Error, Diagnostic, Debug)]
    pub enum Error {
        #[error("Expected more contents")]
        #[diagnostic(code(repkg_build::parser::unexpected_eof))]
        UnexpectedEOF(#[source_code] String, #[label] SourceSpan),
        #[error("Expected project or rule")]
        #[diagnostic(code(repkg_build::parser::invalid_top_level_item))]
        InvalidTopLevelItem {
            #[source_code]
            input: String,
            #[label]
            src: SourceSpan,
        },
        #[error("Lex error")]
        #[diagnostic(code(repkg_build::parser::lex_error))]
        LexError(#[source_code] String, #[label] SourceSpan),
        #[error("Expected token of type {}", .2)]
        #[diagnostic(code(repkg_build::parser::expected_token))]
        ExpectedTokenOfType(
            #[source_code] String,
            #[label("should be {}", .2)] SourceSpan,
            TokenType,
        ),
        #[error("Expected token to be one of: {:?}", .2)]
        #[diagnostic(code(repkg_build::parser::expected_token_of_types))]
        ExpectedTokenOfTypes(
            #[source_code] String,
            #[label("Should be one of {:?}", .2)] SourceSpan,
            Vec<TokenType>,
        ),
    }

    pub(super) fn parse<'a>(lexer: &mut Lexer<'a, Token>, source: &'a str) -> Result<Project> {
        let mut projects = BTreeMap::new();
        let mut rules = BTreeMap::new();

        while let Some(tok) = lexer.peekable().peek() {
            if tok == &Token::Err {
                // Skip whitespace
            } else if &Token::Keyword("project".to_string()) == tok {
                let project = project(lexer, source)?;
                projects.insert(project.name.clone(), project);
            } else if tok.is_type(TokenType::Id) {
                let id = match tok {
                    Token::Id(a) => a,
                    _ => {
                        panic!()
                    }
                }
                .clone();
                let mut rule = rule(lexer, source)?;
                rule.name = id.clone();
                rules.insert(id, rule);
            } else {
                bail!(Error::InvalidTopLevelItem {
                    input: source.to_string(),
                    src: lexer.span().into()
                })
            }
        }

        Ok(Project {
            name: "root".into(),
            projects,
            rules,
            in_: PathBuf::from("."),
        })
    }

    pub(super) fn project<'a>(lexer: &mut Lexer<'a, Token>, source: &'a str) -> Result<Project> {
        let _ = lexer
            .next()
            .ok_or_else(|| unexpected_eof(source))?
            .require_type(TokenType::Keyword("project".to_string()), || {
                Error::ExpectedTokenOfType(
                    source.to_string(),
                    lexer.span().into(),
                    TokenType::Keyword("project".to_string()),
                )
            })?;

        let id = lexer
            .next()
            .ok_or_else(|| unexpected_eof(source))?
            .require_type(TokenType::Id, || {
                Error::ExpectedTokenOfType(source.to_string(), lexer.span().into(), TokenType::Id)
            })?;
        let id = match id {
            Token::Id(id) => id,
            _ => panic!(),
        };
        let mut in_ = PathBuf::from(".");
        if lexer
            .next()
            .ok_or_else(|| unexpected_eof(source))?
            .is_type(TokenType::Keyword("in".to_string()))
        {
            let string = lexer.next().ok_or_else(|| unexpected_eof(source))?;
            dbg!(&string);
            let string = match string {
                Token::String(string) => string,
                _ => bail!(Error::ExpectedTokenOfType(
                    source.to_string(),
                    lexer.span().into(),
                    TokenType::String
                )),
            };
            in_ = PathBuf::from(string);
        }

        let tok = lexer.next().ok_or_else(|| unexpected_eof(source))?;

        let Project {
            name: _,
            projects,
            rules,
            in_: _,
        } = match tok {
            Token::SpecialChar('{') => {
                let project = parse(lexer, source)?;
                let _ = lexer
                    .next()
                    .ok_or_else(|| {
                        Error::ExpectedTokenOfType(
                            format!("{}  ", source),
                            (source.chars().count() + 1..source.chars().count() + 2).into(),
                            TokenType::SpecialChar('}'),
                        )
                    })?
                    .require_type(TokenType::SpecialChar('}'), || {
                        Error::ExpectedTokenOfType(
                            source.to_string(),
                            lexer.span().into(),
                            TokenType::SpecialChar('}'),
                        )
                    });
                project
            }
            Token::SpecialChar(';') => Project {
                name: "".into(),
                projects: BTreeMap::new(),
                rules: BTreeMap::new(),
                in_: PathBuf::new(),
            },
            _ => bail!(ExpectedTokenOfTypes(
                source.to_string(),
                lexer.span().into(),
                vec![TokenType::SpecialChar('{'), TokenType::SpecialChar(';')]
            )),
        };

        Ok(Project {
            name: id,
            projects,
            rules,
            in_,
        })
    }

    pub(super) fn rule<'a>(lexer: &mut Lexer<'a, Token>, source: &'a str) -> Result<Task> {
        let tok = lexer.next().ok_or_else(|| unexpected_eof(source))?;

        let cmds = match tok {
            Token::SpecialChar(':') => vec![command(lexer, source)?],
            _ => bail!(Error::ExpectedTokenOfTypes(
                source.to_string(),
                lexer.span().into(),
                vec![TokenType::SpecialChar('{'), TokenType::SpecialChar(':')]
            )),
        };

        Ok(Task {
            name: "UNNAMED".to_string(),
            cmds,
        })
    }

    pub(super) fn command<'a>(lexer: &mut Lexer<'a, Token>, source: &'a str) -> Result<Command> {
        let tok = lexer.next().ok_or_else(|| unexpected_eof(source))?;
        let id;
        let prefix = match tok {
            Token::Prefix(a) => {
                let id_tok = lexer.next().ok_or_else(|| unexpected_eof(source))?;
                match id_tok {
                    Token::Id(a) => {
                        id = a;
                    }
                    _ => {
                        bail!(Error::ExpectedTokenOfType(
                            source.to_string(),
                            lexer.span().into(),
                            TokenType::Id
                        ))
                    }
                }
                Some(a)
            }
            Token::Id(a) => {
                id = a;
                None
            }
            _ => {
                bail!(Error::ExpectedTokenOfTypes(
                    source.to_string(),
                    lexer.span().into(),
                    vec![
                        TokenType::Prefix('$'),
                        TokenType::Prefix('#'),
                        TokenType::Prefix('!'),
                        TokenType::Id
                    ]
                ))
            }
        };

        let mut args = Vec::new();
        while let Some(tok) = lexer.next() {
            match tok {
                Token::Id(id) => {
                    args.push(id);
                }
                Token::String(s) => {
                    args.push(s);
                }
                Token::Err => break,
                _ => bail!(Error::ExpectedTokenOfType(
                    source.to_string(),
                    lexer.span().into(),
                    TokenType::SpecialChar(';')
                )),
            }
        }

        Ok(Command {
            prefix,
            programs: vec![id],
            args,
        })
    }

    fn unexpected_eof(source: &str) -> Error {
        Error::UnexpectedEOF(
            format!("{}  ", source),
            (source.chars().count() + 1..source.chars().count() + 2).into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use logos::Logos;
    use miette::Result;

    use crate::parser_new::{parser::parse_impl, token::Token};
    use repkg_common::Command;

    #[test]
    fn command() -> Result<()> {
        let program = "cargo test \"build bob\" run --release";
        let command = parse_impl::command(&mut Token::lexer(program), program)?;

        dbg!(&command);

        assert!(command.programs == vec!["cargo"]);
        assert!(command.args == vec!["test", "build bob", "run", "--release"]);
        assert!(command.prefix == None);
        Ok(())
    }

    #[test]
    fn rule() -> Result<()> {
        let program = "build {
            cargo build
            other build
        }";
        let rule = parse_impl::rule(&mut Token::lexer(program), program)?;

        assert!(rule.name == "build".to_string());
        assert!(rule.cmds.len() == 2);
        Ok(())
    }

    #[test]
    fn short_rule() -> Result<()> {
        let program = "build : cargo build";
        let rule = parse_impl::rule(&mut Token::lexer(program), program)?;

        assert!(rule.name == "build".to_string());
        assert!(
            rule.cmds
                == vec![Command {
                    programs: vec!["cargo".to_string()],
                    args: vec!["build".to_string()],
                    prefix: None,
                }]
        );

        Ok(())
    }

    #[test]
    fn project() -> Result<()> {
        let program = "project my-project in \"my-project\" {
            build : $echo blah
        }";
        let project = parse_impl::project(&mut Token::lexer(program), program)?;
        assert!(project.name == "my-project".to_string());
        assert!(project.in_ == PathBuf::from("my-project"));

        Ok(())
    }

    #[test]
    fn project_tasks() -> Result<()> {
        let program = "project my_project {
            build {
                cargo build
            }

            test {
                cargo nextest run
            }
        }";
        let project = parse_impl::project(&mut Token::lexer(program), program)?;

        assert!(project.rules.len() == 2);

        assert!(project
            .rules
            .iter()
            .any(|(name, _rule)| name == &"build".to_string()));
        assert!(project
            .rules
            .iter()
            .any(|(name, _rule)| name == &"test".to_string()));

        Ok(())
    }

    #[test]
    fn many_projects() -> Result<()> {
        let program = "project my_project {
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

        let result = parse_impl::parse(&mut Token::lexer(program), program)?;

        dbg!(&result);

        assert!(result.projects.len() == 2);
        assert!(result.rules.len() == 3);

        Ok(())
    }
}
