use crate::{ast, lex};
use std::{error::Error as StdError, fmt, io};

#[derive(Debug)]
pub enum Error {
    InputRequired,
    Lexer(lex::Error),
    Parser(ast::Error),
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            InputRequired => write!(f, "input required"),
            _ => write!(f, "{}", self),
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

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}
