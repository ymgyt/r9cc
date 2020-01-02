use std::{cell::Cell, error::Error as StdError, fmt, result::Result as StdResult};

#[derive(Debug)]
pub enum ErrorKind {
    InvalidChar(char),
    Eof,
}

pub type Error = Annot<ErrorKind>;
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    fn invalid_char(c: char, loc: Loc) -> Self {
        Error::new(ErrorKind::InvalidChar(c), loc)
    }
    fn eof(loc: Loc) -> Self {
        Error::new(ErrorKind::Eof, loc)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ErrorKind::*;
        match self.value {
            InvalidChar(c) => {
                let padd = " ".repeat(self.loc.0);
                let allow = "^".repeat(self.loc.1 - self.loc.0);
                write!(f, "{}{} invalid char '{}'", padd, allow, c)
            }
            _ => write!(f, "lex error"),
        }
    }
}

impl StdError for Error {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Loc(pub usize, pub usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Annot<T> {
    pub value: T,
    pub loc: Loc,
}

impl<T> Annot<T> {
    fn new(value: T, loc: Loc) -> Self {
        Self { value, loc }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Number(u64), // [0-9][0-9]*
    Plus,        // '+'
    Minus,       // '-'
    Asterisk,    // '*'
    Slash,       // '/'
    LParen,      // '('
    RParen,      // ')'
    Eof,         // sentinel
}

impl TokenKind {
    pub(crate) fn is_number(&self) -> bool {
        match *self {
            TokenKind::Number(_) => true,
            _ => false,
        }
    }
}

pub type Token = Annot<TokenKind>;

impl Token {
    pub(crate) fn number(n: u64, loc: Loc) -> Self {
        Self::new(TokenKind::Number(n), loc)
    }
    pub(crate) fn plus(loc: Loc) -> Self {
        Self::new(TokenKind::Plus, loc)
    }
    pub(crate) fn minus(loc: Loc) -> Self {
        Self::new(TokenKind::Minus, loc)
    }
    pub(crate) fn asterisk(loc: Loc) -> Self {
        Self::new(TokenKind::Asterisk, loc)
    }
    pub(crate) fn slash(loc: Loc) -> Self {
        Self::new(TokenKind::Slash, loc)
    }
    pub(crate) fn lparen(loc: Loc) -> Self {
        Self::new(TokenKind::LParen, loc)
    }
    pub(crate) fn rparen(loc: Loc) -> Self {
        Self::new(TokenKind::RParen, loc)
    }
    fn eof(loc: Loc) -> Self {
        Self::new(TokenKind::Eof, loc)
    }
    pub(crate) fn is_kind(&self, kind: TokenKind) -> bool {
        match self.value {
            TokenKind::Number(_) => kind.is_number(),
            _ => self.value == kind,
        }
    }
}

pub type Stream = Vec<Token>;

#[derive(Debug)]
struct Input<'a> {
    input: &'a [u8],
    pos: Cell<usize>,
}

impl<'a> Input<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            input: s.as_bytes(),
            pos: Cell::new(0),
        }
    }
    fn consume_byte(&self, want: u8) -> Result<usize> {
        self.peek().and_then(|got: u8| {
            if got != want {
                let pos = self.pos();
                Err(Error::invalid_char(got as char, Loc(pos, pos + 1)))
            } else {
                Ok(self.pos_then_inc())
            }
        })
    }
    fn consume_numbers(&self) -> Result<(usize, u64)> {
        let start = self.pos();
        self.consume(|b| b"0123456789".contains(&b));
        let n = std::str::from_utf8(&self.input[start..self.pos()])
            .unwrap()
            .parse()
            .unwrap();
        Ok((start, n))
    }
    fn consume_spaces(&self) {
        self.consume(|b| b" \n\t".contains(&b))
    }
    fn consume(&self, mut f: impl FnMut(u8) -> bool) {
        while let Ok(b) = self.peek() {
            if f(b) {
                self.inc();
                continue;
            }
            break;
        }
    }
    fn peek(&self) -> Result<u8> {
        self.input
            .get(self.pos())
            .map(|&b| b)
            .ok_or_else(|| self.eof())
    }
    fn pos(&self) -> usize {
        self.pos.get()
    }
    fn inc(&self) {
        self.pos.set(self.pos() + 1);
    }
    fn pos_then_inc(&self) -> usize {
        let pos = self.pos();
        self.inc();
        pos
    }
    fn eof(&self) -> Error {
        let pos = self.pos();
        Error::eof(Loc(pos, pos))
    }
}

pub fn tokenize(input: &str) -> StdResult<Stream, crate::Error> {
    let mut tokens = Vec::new();
    let input = Input::new(input);

    macro_rules! push {
        ($lexer:expr) => {{
            let tk = $lexer?;
            tokens.push(tk);
        }};
    }
    loop {
        match input.peek() {
            Err(e) => match e.value {
                ErrorKind::Eof => {
                    tokens.push(Token::eof(Loc(input.pos(), input.pos())));
                    return Ok(tokens);
                }
                _ => return Err(e.into()),
            },
            Ok(b) => match b {
                b'0'..=b'9' => push!(lex_number(&input)),
                b'+' => push!(lex_plus(&input)),
                b'-' => push!(lex_minus(&input)),
                b'*' => push!(lex_asterisk(&input)),
                b'/' => push!(lex_slash(&input)),
                b'(' => push!(lex_lparen(&input)),
                b')' => push!(lex_rparen(&input)),
                _ if (b as char).is_ascii_whitespace() => input.consume_spaces(),
                _ => {
                    return Err(
                        Error::invalid_char(b as char, Loc(input.pos(), input.pos() + 1)).into(),
                    )
                }
            },
        }
    }
}

fn lex_number(input: &Input) -> Result<Token> {
    input
        .consume_numbers()
        .map(|(pos, n)| Token::number(n, Loc(pos, input.pos())))
}

fn lex_plus(input: &Input) -> Result<Token> {
    input
        .consume_byte(b'+')
        .map(|pos| Token::plus(Loc(pos, pos + 1)))
}

fn lex_minus(input: &Input) -> Result<Token> {
    input
        .consume_byte(b'-')
        .map(|pos| Token::minus(Loc(pos, pos + 1)))
}

fn lex_asterisk(input: &Input) -> Result<Token> {
    input
        .consume_byte(b'*')
        .map(|pos| Token::asterisk(Loc(pos, pos + 1)))
}

fn lex_slash(input: &Input) -> Result<Token> {
    input
        .consume_byte(b'/')
        .map(|pos| Token::slash(Loc(pos, pos + 1)))
}

fn lex_lparen(input: &Input) -> Result<Token> {
    input
        .consume_byte(b'(')
        .map(|pos| Token::lparen(Loc(pos, pos + 1)))
}

fn lex_rparen(input: &Input) -> Result<Token> {
    input
        .consume_byte(b')')
        .map(|pos| Token::rparen(Loc(pos, pos + 1)))
}

#[cfg(test)]
#[path = "./token_test.rs"]
mod token_test;
