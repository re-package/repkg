use std::fmt::{Debug, Display};

use logos::Logos;
use miette::{bail, Result};

#[derive(Logos, Debug, PartialEq, Eq)]
pub enum Token {
    #[regex("[0-9]*", |lex| lex.slice().parse())]
    Number(u64),
    #[regex("\"[^\"]*\"", |lex| lex.slice().to_string())]
    String(String),
    #[regex("[-_a-zA-Z]+", |lex| lex.slice().to_string())]
    Id(String),
    #[regex("[{};:]", |lex| lex.slice().chars().next().unwrap())]
    SpecialChar(char),
    #[regex("[$#!]", |lex| lex.slice().chars().next().unwrap())]
    Prefix(char),
    #[regex(r"(project)|(at)|(in)", |lex| lex.slice().to_string())]
    Keyword(String),
    #[regex(r"[ \n\t\f]+", logos::skip)]
    #[error]
    Err,
}

impl Token {
    pub fn is_type(&self, type_: TokenType) -> bool {
        type_
            == match self {
                Token::Number(_) => TokenType::Number,
                Token::String(_) => TokenType::String,
                Token::Id(_) => TokenType::Id,
                Token::SpecialChar(a) => TokenType::SpecialChar(*a),
                Token::Err => TokenType::Err,
                Token::Keyword(a) => TokenType::Keyword(a.to_string()),
                Token::Prefix(a) => TokenType::Prefix(*a),
            }
    }

    pub fn require_type<T: Into<miette::ErrReport>>(
        self,
        type_: TokenType,
        mk_err: impl Fn() -> T,
    ) -> Result<Self> {
        if self.is_type(type_) {
            Ok(self)
        } else {
            bail!(mk_err())
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum TokenType {
    Number,
    String,
    Id,
    SpecialChar(char),
    Prefix(char),
    Keyword(String),
    Err,
}

impl Debug for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TokenType::Number => "Number".to_string(),
                TokenType::String => "String".to_string(),
                TokenType::Id => "Id".to_string(),
                TokenType::SpecialChar(a) => format!("'{}'", a),
                TokenType::Err => "Err".to_string(),
                TokenType::Keyword(a) => format!("'{}'", a),
                TokenType::Prefix(a) => format!("'{}'", a),
            }
        )
    }
}
