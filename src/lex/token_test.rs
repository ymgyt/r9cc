use super::*;

#[test]
fn tokenize_test() {
    let s = tokenize("+100+").unwrap();
    assert_eq!(
        s,
        vec![
            Token::plus(Loc(0, 1)),
            Token::number(100, Loc(1, 4)),
            Token::plus(Loc(4, 5)),
            Token::eof(Loc(5, 5)),
        ]
    );

    let s = tokenize("+-*/()").unwrap();
    assert_eq!(
        s,
        vec![
            Token::plus(Loc(0, 1)),
            Token::minus(Loc(1, 2)),
            Token::asterisk(Loc(2, 3)),
            Token::slash(Loc(3, 4)),
            Token::lparen(Loc(4, 5)),
            Token::rparen(Loc(5, 6)),
            Token::eof(Loc(6, 6)),
        ]
    );

    let s = tokenize(" 100\t200\n300 +  \n").unwrap();
    assert_eq!(
        s,
        vec![
            Token::number(100, Loc(1, 4)),
            Token::number(200, Loc(5, 8)),
            Token::number(300, Loc(9, 12)),
            Token::plus(Loc(13, 14)),
            Token::eof(Loc(17, 17)),
        ]
    );
}

#[test]
fn comparison_operator_test() {
    assert_eq!(
        tokenize("==").unwrap(),
        vec![Token::equal(Loc(0, 2)), Token::eof(Loc(2, 2))],
    );
    assert!(match tokenize("=") {
        Err(crate::Error::Lexer(e)) => e.value == ErrorKind::InvalidChar('='),
        _ => false,
    });
    assert_eq!(
        tokenize("!=").unwrap(),
        vec![Token::not_equal(Loc(0, 2)), Token::eof(Loc(2, 2))],
    );
    assert_eq!(
        tokenize(">=").unwrap(),
        vec![Token::greater_equal(Loc(0, 2)), Token::eof(Loc(2, 2))],
    );
    assert_eq!(
        tokenize(">").unwrap(),
        vec![Token::greater_than(Loc(0, 1)), Token::eof(Loc(1, 1))],
    );
    assert_eq!(
        tokenize("<=").unwrap(),
        vec![Token::less_equal(Loc(0, 2)), Token::eof(Loc(2, 2))],
    );
    assert_eq!(
        tokenize("<").unwrap(),
        vec![Token::less_than(Loc(0, 1)), Token::eof(Loc(1, 1))],
    );

    let s = tokenize("== != >= > <= <").unwrap();
    assert_eq!(
        s,
        vec![
            Token::equal(Loc(0, 2)),
            Token::not_equal(Loc(3, 5)),
            Token::greater_equal(Loc(6, 8)),
            Token::greater_than(Loc(9, 10)),
            Token::less_equal(Loc(11, 13)),
            Token::less_than(Loc(14, 15)),
            Token::eof(Loc(15, 15)),
        ]
    );
}
