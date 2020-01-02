use r9cc::{gen, parse, tokenize, Error};
use std::{env, io, process};

fn main() {
    let result = env::args()
        .skip(1)
        .next()
        .ok_or(Error::InputRequired)
        .and_then(|input| tokenize(&input))
        .and_then(|tokens| parse(tokens))
        .and_then(|ast| gen(&mut io::stdout(), &ast));

    if let Err(e) = result {
        match e {
            Error::Lexer(e) => {
                eprintln!("{}\n{}", env::args().skip(1).next().unwrap(), e);
            }
            _ => eprintln!("{}", e),
        }

        process::exit(1);
    }
}
