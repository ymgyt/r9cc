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
    assert_eq!(
        node,
        Node::new(
            Kind::Add,
            Node::link(Node::number(10)),
            Node::link(Node::number(20)),
        ),
    );

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
        Node::new(
            Kind::Mul,
            Node::link(Node::number(1)),
            Node::link(Node::new(
                Kind::Add,
                Node::link(Node::number(2)),
                Node::link(Node::number(3)),
            )),
        ),
    );
}
