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
    let mut tokens = stream.into_iter().peekable();
    expr(&mut tokens).map_err(|e| crate::Error::from(e))
}

// expr = mul ( "+" mul | "-" mul )*
fn expr<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Node>
where
    Tokens: Iterator<Item = Token>,
{
    let mut node = mul(tokens)?;
    loop {
        if consume(tokens, TokenKind::Plus)? {
            node = Node::with(Kind::Add, node, mul(tokens)?);
        } else if consume(tokens, TokenKind::Minus)? {
            node = Node::with(Kind::Sub, node, mul(tokens)?)
        } else {
            return Ok(node);
        }
    }
}

// mul = unary ( "*" unary | "/" unary)*
fn mul<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Node>
where
    Tokens: Iterator<Item = Token>,
{
    let mut node = unary(tokens)?;
    loop {
        if consume(tokens, TokenKind::Asterisk)? {
            node = Node::with(Kind::Mul, node, unary(tokens)?);
        } else if consume(tokens, TokenKind::Slash)? {
            node = Node::with(Kind::Div, node, unary(tokens)?);
        } else {
            return Ok(node);
        }
    }
}

// unary = ("+" | "-")? unary
fn unary<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Node>
where
    Tokens: Iterator<Item = Token>,
{
    let node = if consume(tokens, TokenKind::Plus)? {
        primary(tokens)?
    } else if consume(tokens, TokenKind::Minus)? {
        Node::ops(Kind::Sub, 0, expect_number(tokens)?)
    } else {
        primary(tokens)?
    };
    Ok(node)
}

// primary = "(" expr ")" | num
fn primary<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Node>
where
    Tokens: Iterator<Item = Token>,
{
    let node = if consume(tokens, TokenKind::LParen)? {
        let node = expr(tokens)?;
        expect(tokens, TokenKind::RParen)?;
        node
    } else {
        Node::number(expect_number(tokens)?)
    };
    Ok(node)
}

fn consume<Tokens>(tokens: &mut Peekable<Tokens>, kind: TokenKind) -> Result<bool>
where
    Tokens: Iterator<Item = Token>,
{
    // how can i make this code to a method chain :(
    let peek = tokens.peek();
    if peek.is_none() {
        return Ok(false);
    }
    let peek = peek.unwrap();
    if peek.is_kind(kind) {
        tokens.next();
        Ok(true)
    } else {
        Ok(false)
    }
}

fn expect<Tokens>(tokens: &mut Peekable<Tokens>, kind: TokenKind) -> Result<()>
where
    Tokens: Iterator<Item = Token>,
{
    tokens
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
            tokens.next();
            unit
        })
}

fn expect_number<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<u64>
where
    Tokens: Iterator<Item = Token>,
{
    tokens
        .peek()
        .ok_or(Error::Eof)
        .and_then(|peek| match peek.value {
            TokenKind::Number(n) => Ok(n),
            _ => Err(Error::UnexpectedToken(peek.clone())),
        })
        .map(|n| {
            tokens.next();
            n
        })
}

#[cfg(test)]
#[path = "./parser_test.rs"]
mod parser_test;
