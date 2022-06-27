use std::collections::HashMap;
use crate::risp::AstNode;
use super::{Interpreter, Type, RuntimeError, ErrorKind};

macro_rules! err {
    ($kind:ident, $msg:expr) => {
        Err(RuntimeError {
            kind: ErrorKind::$kind,
            msg: $msg.into()
        })
    };
}

fn set(inter: &mut Interpreter, mut nodes: Vec<AstNode>) -> Result<Type, RuntimeError> {
    if nodes.len() != 2 {
        return err!(ValueError, format!("expected 2 arguments, found {}", nodes.len()));
    }

    if let AstNode::Name(name) = nodes.remove(0) {
        let value = inter.eval(nodes.pop().unwrap()).unwrap();
        inter.set_name(&name, value.clone());
        Ok(value)
    } else {
        return err!(ValueError, format!("first argument must eb a name"))
    }
}

fn list(inter: &mut Interpreter, nodes: Vec<AstNode>) -> Result<Type, RuntimeError> {
    let mut elems: Vec<Type> = Vec::new();
    for node in nodes {
        elems.push(inter.eval(node)?);
    };

    Ok(elems.into())
}

lazy_static! {
    pub static ref SYMBOLS: HashMap<String, Type> = {
        let mut h = HashMap::new();
        h.insert("set".into(), Type::RustMacro(set));
        h.insert("list".into(), Type::RustMacro(list));
        h
    };
}
