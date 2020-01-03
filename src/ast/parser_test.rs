use super::*;
use crate::lex::token::Loc;

#[test]
fn parse_test() {
    // 10+20
    let s = vec![
        Token::number(10, Loc(0, 1)),
        Token::plus(Loc(1, 2)),
        Token::number(20, Loc(2, 3)),
    ];

    let node = parse(s).unwrap();
    assert_eq!(node, Node::ops(Kind::Add, 10, 20),);

    // 1*(2+3)
    let s = vec![
        Token::number(1, Loc(0, 1)),
        Token::asterisk(Loc(1, 2)),
        Token::lparen(Loc(2, 3)),
        Token::number(2, Loc(3, 4)),
        Token::plus(Loc(4, 5)),
        Token::number(3, Loc(5, 6)),
        Token::rparen(Loc(6, 7)),
    ];

    let node = parse(s).unwrap();
    assert_eq!(
        node,
        Node::with(Kind::Mul, Node::number(1), Node::ops(Kind::Add, 2, 3),),
    );
}

#[test]
fn unary_test() {
    // -3*+5
    let s = vec![
        Token::minus(Loc(0, 1)),
        Token::number(3, Loc(1, 2)),
        Token::asterisk(Loc(2, 3)),
        Token::plus(Loc(3, 4)),
        Token::number(5, Loc(4, 5)),
    ];
    let node = parse(s).unwrap();
    assert_eq!(
        node,
        Node::with(Kind::Mul, Node::ops(Kind::Sub, 0, 3), Node::number(5),)
    );
}

#[test]
fn comparison_operator_test() {
    // '1 == 3'
    let s = vec![
        Token::number(1, Loc(0, 1)),
        Token::equal(Loc(2, 4)),
        Token::number(3, Loc(5, 6)),
    ];
    assert_eq!(parse(s).unwrap(), Node::ops(Kind::Eq, 1, 3));

    // '1 != 3'
    let s = vec![
        Token::number(1, Loc(0, 1)),
        Token::not_equal(Loc(2, 4)),
        Token::number(3, Loc(5, 6)),
    ];
    assert_eq!(parse(s).unwrap(), Node::ops(Kind::Ne, 1, 3));

    // '1 >= 3'
    let s = vec![
        Token::number(1, Loc(0, 1)),
        Token::greater_equal(Loc(2, 4)),
        Token::number(3, Loc(5, 6)),
    ];
    assert_eq!(parse(s).unwrap(), Node::ops(Kind::Le, 3, 1));

    // '1 > 3'
    let s = vec![
        Token::number(1, Loc(0, 1)),
        Token::greater_than(Loc(2, 3)),
        Token::number(3, Loc(4, 5)),
    ];
    assert_eq!(parse(s).unwrap(), Node::ops(Kind::Lt, 3, 1));

    // '1 <= 3'
    let s = vec![
        Token::number(1, Loc(0, 1)),
        Token::less_equal(Loc(2, 4)),
        Token::number(3, Loc(5, 6)),
    ];
    assert_eq!(parse(s).unwrap(), Node::ops(Kind::Le, 1, 3));

    // '1 < 3'
    let s = vec![
        Token::number(1, Loc(0, 1)),
        Token::less_than(Loc(2, 3)),
        Token::number(3, Loc(4, 5)),
    ];
    assert_eq!(parse(s).unwrap(), Node::ops(Kind::Lt, 1, 3));
}
