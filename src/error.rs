use crate::{ast, lex, asm};
use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub enum Error {
    InputRequired,
    Lexer(lex::Error),
    Parser(ast::Error),
    Asm(asm::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            InputRequired => write!(f, "input required"),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        use Error::*;
        match *self {
            Lexer(ref e) => Some(e),
            Parser(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<lex::Error> for Error {
    fn from(e: lex::Error) -> Self {
        Error::Lexer(e)
    }
}

impl From<ast::Error> for Error {
    fn from(e: ast::Error) -> Self {
        Error::Parser(e)
    }
}

impl From<asm::Error> for Error {
    fn from(e: asm::Error) -> Self {
         Error::Asm(e)
    }
}
