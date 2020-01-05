use r9cc::{generate, parse, tokenize, Error};
use std::{env, io, process};

fn main() {
    let result = env::args()
        .nth(1)
        .ok_or(Error::InputRequired)
        .map(|mut input| {
            if !input.ends_with(";") {
                input.push(';');
            }
            input
        })
        .and_then(|input| tokenize(&input))
        .and_then(|tokens| parse(tokens))
        .and_then(|program| generate(&mut io::stdout(), program));

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
