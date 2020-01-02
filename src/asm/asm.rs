use crate::ast::{Kind as NodeKind, Node};
use std::io::{self, Write};

pub fn gen<W: Write>(w: &mut W, node: &Node) -> Result<(), crate::Error> {
    pre_gen(w)
        .and_then(|_| main_gen(w, node))
        .and_then(|_| post_gen(w))
        .map_err(|e| crate::Error::from(e))
}

fn pre_gen<W: Write>(w: &mut W) -> io::Result<()> {
    write!(
        w,
        ".intel_syntax noprefix\n\
         .global main\n\
         main:\n",
    )
}

fn main_gen<W: Write>(w: &mut W, node: &Node) -> io::Result<()> {
    if let NodeKind::Number(n) = node.kind {
        write!(w, "  push {}\n", n)?;
        return Ok(());
    }

    main_gen(w, &node.lhs.as_ref().unwrap())?;
    main_gen(w, &node.rhs.as_ref().unwrap())?;

    write!(w, "  pop rdi\n")?;
    write!(w, "  pop rax\n")?;

    match node.kind {
        NodeKind::Add => write!(w, "  add rax, rdi\n")?,
        NodeKind::Sub => write!(w, "  sub rax, rdi\n")?,
        NodeKind::Mul => write!(w, "  imul rax, rdi\n")?,
        NodeKind::Div => {
            write!(w, "  cqo\n")?;
            write!(w, "  idiv rdi\n")?;
        }
        _ => unreachable!(),
    }

    write!(w, "  push rax\n")?;

    Ok(())
}

fn post_gen<W: Write>(w: &mut W) -> io::Result<()> {
    write!(w, "{}{}", "  pop rax\n", "  ret\n")
}
