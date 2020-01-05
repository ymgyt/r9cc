pub type Program = Vec<Node>;

#[derive(Debug, PartialEq, Clone)]
pub enum Kind {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Lt,
    Le,
    Assign,
    LocalVar(LocalVar),
    Number(u64),
}

#[derive(Debug, PartialEq, Clone)]
pub struct LocalVar {
    pub offset: u64, // offset from base pointer
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
    pub fn with(kind: Kind, lhs: Node, rhs: Node) -> Node {
        Node::new(kind, Node::link(lhs), Node::link(rhs))
    }
    pub fn ops(kind: Kind, lhs: u64, rhs: u64) -> Node {
        use Kind::*;
        match kind {
            Add | Sub | Mul | Div | Eq | Ne | Le | Lt => {
                Node::with(kind, Node::number(lhs), Node::number(rhs))
            }
            _ => panic!("operation kind required. got {:?}", kind),
        }
    }
    pub fn link(node: Node) -> Link {
        Some(Box::new(node))
    }
    pub fn number(n: u64) -> Node {
        Node::new(Kind::Number(n), None, None)
    }
    pub fn local_var(offset: u64) -> Node {
        Node::new(Kind::LocalVar(LocalVar{offset}), None, None)
    }
}

