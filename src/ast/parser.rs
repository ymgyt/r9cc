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
            node = Node::new(Kind::Add, Node::link(node), Node::link(mul(tokens)?));
        } else if consume(tokens, TokenKind::Minus)? {
            node = Node::new(Kind::Sub, Node::link(node), Node::link(mul(tokens)?));
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
            node = Node::new(Kind::Mul, Node::link(node), Node::link(unary(tokens)?));
        } else if consume(tokens, TokenKind::Slash)? {
            node = Node::new(Kind::Div, Node::link(node), Node::link(unary(tokens)?));
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
    let peek = tokens.peek();
    if peek.is_none() {
        return Err(Error::Eof);
    }
    let peek = peek.unwrap();
    if peek.is_kind(kind) {
        tokens.next();
        Ok(())
    } else {
        Err(Error::UnexpectedToken(peek.clone()))
    }
}

fn expect_number<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<u64>
where
    Tokens: Iterator<Item = Token>,
{
    let peek = tokens.peek();
    if peek.is_none() {
        return Err(Error::Eof);
    }
    let peek = peek.unwrap();
    match peek.value {
        TokenKind::Number(n) => {
            tokens.next();
            Ok(n)
        }
        _ => Err(Error::UnexpectedToken(peek.clone())),
    }
}

#[cfg(test)]
#[path = "./parser_test.rs"]
mod parser_test;
