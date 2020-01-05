use crate::ast::{Kind as NodeKind, Node,Program};
use std::{fmt, io::{self, Write}, result::Result as StdResult};

#[derive(Debug)]
pub enum Error {
   Write(io::Error),
    UnexpectedNode(NodeKind),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
       match self {
           Error::Write(e) => write!(f,"{}", e),
           Error::UnexpectedNode(kind) => write!(f, "unexpected node {:?}", kind),
       }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Write(e)
    }
}

type Result<T> = StdResult<T, Error>;

pub fn generate<W: Write>(w: &mut W, program: Program) -> StdResult<(), crate::Error> {
    pre_gen(w)
        .and(prologue(w))
        .and(main_gen(w, program))
        .and( epilogue(w))
        .map_err(|e| crate::Error::from(e))
}

fn pre_gen<W: Write>(w: &mut W) -> Result<()> {
    write!(
        w,
        ".intel_syntax noprefix\n\
         .global main\n\
         main:\n",
    )?;
    Ok(())
}

fn prologue<W: Write>(w: &mut W) -> Result<()> {
    write!(w, "  push rbp\n")?;
    write!(w, "  mov rbp, rsp\n")?;
    write!(w, "  sub rsp, 208\n")?;
    Ok(())
}

fn main_gen<W: Write>(w: &mut W, program: Program) -> Result<()> {
    for node in &program {
        gen(w, node)?;
        write!(w, "  pop rax\n")?;
    }
    Ok(())
}

fn gen_local_var<W: Write>(w: &mut W, node: &Node) -> Result<()> {
    if let NodeKind::LocalVar(lv) = &node.kind {
        write!(w, "  mov rax, rbp\n")?;
        write!(w, "  sub rax, {}\n", lv.offset)?;
        write!(w, "  push rax\n")?;
        Ok(())
    } else {
        Err(Error::UnexpectedNode(node.kind.clone()))
    }
}

fn gen<W: Write>(w: &mut W, node: &Node) -> Result<()> {
    match node.kind {
        NodeKind::Number(n) => {
            write!(w, "  push {}\n", n)?;
            return Ok(());
        },
        NodeKind::LocalVar(_) => {
            gen_local_var(w, node)?;
            write!(w, "  pop rax\n")?;
            write!(w, "  mov rax, [rax]\n")?;
            write!(w, "  push rax\n")?;
            return Ok(());
        },
        NodeKind::Assign => {
            gen_local_var(w, node.lhs.as_ref().unwrap())?;
            gen(w, node.rhs.as_ref().unwrap())?;
            write!(w, "  pop rdi\n")?;
            write!(w, "  pop rax\n")?;
            write!(w, "  mov [rax], rdi\n")?;
            write!(w, "  push rdi\n")?;
            return Ok(());
        },
        _ => (),
    }

    gen(w, &node.lhs.as_ref().unwrap())?;
    gen(w, &node.rhs.as_ref().unwrap())?;

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
        NodeKind::Eq => write!(
            w,
            "  {}",
            "cmp rax, rdi\n  \
             sete al\n  \
             movzb rax, al\n",
        )?,
        NodeKind::Ne => write!(
            w,
            "  {}",
            "cmp rax, rdi\n  \
             setne al\n  \
             movzb rax, al\n",
        )?,
        NodeKind::Lt => write!(
            w,
            "  {}",
            "cmp rax, rdi\n  \
             setl al\n  \
             movzb rax, al\n",
        )?,
        NodeKind::Le => write!(
            w,
            "  {}",
            "cmp rax, rdi\n  \
             setle al\n  \
             movzb rax, al\n",
        )?,
        NodeKind::Number(_) => unreachable!(),
        _ => unimplemented!()
    }

    write!(w, "  push rax\n")?;

    Ok(())
}

fn epilogue<W: Write>(w: &mut W) -> Result<()> {
    write!(w, "  mov rsp, rbp\n")?;
    write!(w, "  pop rbp\n")?;
    write!(w, "  ret\n")?;
    Ok(())
}
