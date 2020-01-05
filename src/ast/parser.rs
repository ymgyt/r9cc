use crate::{
    ast::node::{Kind, Node, Program},
    lex::{Stream, Token, TokenKind, Ident},
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

pub fn parse(stream: Stream) -> StdResult<Program, crate::Error> {
    Parser::new(stream.into_iter().peekable())
        .program()
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

    // program = stmt *
    fn program(&mut self) -> Result<Program> {
        let mut program = Program::new();
        while !self.is_eof() {
            program.push(self.stmt()?);
        }
        Ok(program)
    }

    // stmt = expr ";"
    fn stmt(&mut self) -> Result<Node> {
        let node = self.expr()?;
        self.expect(TokenKind::SemiColon)?;
        Ok(node)
    }

    // expr = assign
    fn expr(&mut self) -> Result<Node> {
        self.assign()
    }

    // assign = equality ( "=" assign )*
    fn assign(&mut self) -> Result<Node> {
        let mut node = self.equality()?;
        while self.consume(TokenKind::Assign)? {
            node = Node::with(Kind::Assign, node, self.assign()?);
        }
        Ok(node)
    }

    // equality = relational ("==" relational | "!=" relational)*
    fn equality(&mut self) -> Result<Node> {
        let mut node = self.relational()?;
        loop {
            if self.consume(TokenKind::Eq)? {
                node = Node::with(Kind::Eq, node, self.relational()?);
            } else if self.consume(TokenKind::Ne)? {
                node = Node::with(Kind::Ne, node, self.relational()?)
            } else {
                return Ok(node);
            }
        }
    }

    // relational = add ("<" add | "<=" add | ">" add | ">=" add)*
    fn relational(&mut self) -> Result<Node> {
        let mut node = self.add()?;
        loop {
            if self.consume(TokenKind::Lt)? {
                node = Node::with(Kind::Lt, node, self.add()?);
            } else if self.consume(TokenKind::Le)? {
                node = Node::with(Kind::Le, node, self.add()?);
            } else if self.consume(TokenKind::Gt)? {
                node = Node::with(Kind::Lt, self.add()?, node);
            } else if self.consume(TokenKind::Ge)? {
                node = Node::with(Kind::Le, self.add()?, node);
            } else {
                return Ok(node);
            }
        }
    }

    // add = mul ("+" mul | "-" mul)*
    fn add(&mut self) -> Result<Node> {
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

    // mul = unary ("*" unary | "/" unary)*
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

    // unary = ("+" | "-")? primary
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

    // primary = num | ident | "(" expr ")"
    fn primary(&mut self) -> Result<Node> {
        let node = if self.consume(TokenKind::LParen)? {
            let node = self.expr()?;
            self.expect(TokenKind::RParen)?;
            node
        } else if self.is_ident() {
            let ident = self.expect_ident()?;
            let offset = ((ident.name.chars().next().unwrap() as u8) - b'a' + 1) * 8;
            Node::local_var(offset.into())
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
    fn expect_ident(&mut self) -> Result<Ident> {
        self.tokens.peek().ok_or(Error::Eof).and_then(|peek| match peek.value {
            TokenKind::Ident(ref ident) => Ok(ident.clone()),
            _ => Err(Error::UnexpectedToken(peek.clone())),
        }).map(|ident| {
            self.tokens.next();
            ident
        })
    }
    fn is_ident(&mut self) -> bool {
       self.tokens.peek().map_or(false, |peek| peek.is_ident())
    }
    fn is_eof(&mut self) -> bool {
        self.tokens
            .peek()
            .map_or(true, |peek| peek.is_kind(TokenKind::Eof))
    }
}

#[cfg(test)]
#[path = "./parser_test.rs"]
mod parser_test;
