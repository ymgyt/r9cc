use crate::{
    ast::node::{Kind, Node},
    lex::{Stream, Token, TokenKind},
};
use std::{error::Error as StdError, fmt, iter::Peekable, result::Result as StdResult};

#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedToken(Token),
    Eof,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parser error")
    }
}

impl StdError for Error {}

type Result<T> = StdResult<T, Error>;

pub fn parse(stream: Stream) -> StdResult<Node, crate::Error> {
    Parser::new(stream.into_iter().peekable())
        .expr()
        .map_err(|e| crate::Error::from(e))
}

struct Parser<Tokens> {
    tokens: Tokens,
}

impl<Tokens> Parser<Peekable<Tokens>>
where
    Tokens: Iterator<Item = Token>,
{
    fn new(tokens: Peekable<Tokens>) -> Self {
        Self { tokens }
    }

    fn expr(&mut self) -> Result<Node> {
        let mut node = self.mul()?;
        loop {
            if self.consume(TokenKind::Plus)? {
                node = Node::with(Kind::Add, node, self.mul()?);
            } else if self.consume(TokenKind::Minus)? {
                node = Node::with(Kind::Sub, node, self.mul()?)
            } else {
                return Ok(node);
            }
        }
    }

    fn mul(&mut self) -> Result<Node> {
        let mut node = self.unary()?;
        loop {
            if self.consume(TokenKind::Asterisk)? {
                node = Node::with(Kind::Mul, node, self.unary()?);
            } else if self.consume(TokenKind::Slash)? {
                node = Node::with(Kind::Div, node, self.unary()?);
            } else {
                return Ok(node);
            }
        }
    }

    fn unary(&mut self) -> Result<Node> {
        let node = if self.consume(TokenKind::Plus)? {
            self.primary()?
        } else if self.consume(TokenKind::Minus)? {
            Node::ops(Kind::Sub, 0, self.expect_number()?)
        } else {
            self.primary()?
        };
        Ok(node)
    }

    fn primary(&mut self) -> Result<Node> {
        let node = if self.consume(TokenKind::LParen)? {
            let node = self.expr()?;
            self.expect(TokenKind::RParen)?;
            node
        } else {
            Node::number(self.expect_number()?)
        };
        Ok(node)
    }

    fn consume(&mut self, kind: TokenKind) -> Result<bool> {
        // how can i make this code to a method chain :(
        let peek = self.tokens.peek();
        if peek.is_none() {
            return Ok(false);
        }
        let peek = peek.unwrap();
        if peek.is_kind(kind) {
            self.tokens.next();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn expect(&mut self, kind: TokenKind) -> Result<()> {
        self.tokens
            .peek()
            .ok_or(Error::Eof)
            .and_then(|peek| {
                if peek.is_kind(kind) {
                    Ok(())
                } else {
                    Err(Error::UnexpectedToken(peek.clone()))
                }
            })
            .map(|unit| {
                self.tokens.next();
                unit
            })
    }

    fn expect_number(&mut self) -> Result<u64> {
        self.tokens
            .peek()
            .ok_or(Error::Eof)
            .and_then(|peek| match peek.value {
                TokenKind::Number(n) => Ok(n),
                _ => Err(Error::UnexpectedToken(peek.clone())),
            })
            .map(|n| {
                self.tokens.next();
                n
            })
    }
}

#[cfg(test)]
#[path = "./parser_test.rs"]
mod parser_test;
