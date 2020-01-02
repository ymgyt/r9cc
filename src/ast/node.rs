#[derive(Debug, PartialEq)]
pub enum Kind {
    Add,
    Sub,
    Mul,
    Div,
    Number(u64),
}

pub type Link = Option<Box<Node>>;

#[derive(Debug, PartialEq)]
pub struct Node {
    pub kind: Kind,
    pub lhs: Link,
    pub rhs: Link,
}

impl Node {
    pub fn new(kind: Kind, lhs: Link, rhs: Link) -> Node {
        Self { kind, lhs, rhs }
    }
    pub fn link(node: Node) -> Link {
        Some(Box::new(node))
    }
    pub fn number(n: u64) -> Node {
        Node::new(Kind::Number(n), None, None)
    }
}
