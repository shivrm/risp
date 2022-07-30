use super::{ErrorKind, Interpreter, RuntimeError, WrappedType};
use crate::risp::AstNode;
use std::collections::HashMap;

macro_rules! err {
    ($kind:ident, $msg:expr) => {
        Err(RuntimeError {
            kind: ErrorKind::$kind,
            msg: $msg.into(),
        })
    };
}

fn set(inter: &mut Interpreter, nodes: &[AstNode]) -> Result<WrappedType, RuntimeError> {
    if nodes.len() != 2 {
        return err!(
            ValueError,
            format!("expected 2 arguments, found {}", nodes.len())
        );
    }

    if let AstNode::Name(name) = &nodes[0] {
        let value = inter.eval(&nodes[1]).unwrap();
        inter.set_name(&name, value.clone());
        Ok(value)
    } else {
        return err!(ValueError, format!("first argument must eb a name"));
    }
}

fn list(inter: &mut Interpreter, nodes: &[AstNode]) -> Result<WrappedType, RuntimeError> {
    let mut elems: Vec<WrappedType> = Vec::new();
    for node in nodes {
        elems.push(inter.eval(&node)?);
    }

    Ok(elems.into())
}

fn block(inter: &mut Interpreter, nodes: &[AstNode]) -> Result<WrappedType, RuntimeError> {
    let mut res = WrappedType::Null;
    for node in nodes {
        res = inter.eval(&node)?;
    }

    Ok(res)
}

fn if_else(inter: &mut Interpreter, nodes: &[AstNode]) -> Result<WrappedType, RuntimeError> {
    let has_else;
    match nodes.len() {
        2 => has_else = false,
        3 => has_else = true,
        _ => return err!(ValueError, "Incorrect number of arguments"),
    };

    let cond = &nodes[0];
    let if_expr = &nodes[1];

    if let WrappedType::Bool(true) = inter.eval(cond)? {
        inter.eval(if_expr)
    } else {
        if has_else == false {
            return Ok(WrappedType::Null);
        }

        let else_expr = &nodes[2];
        inter.eval(&else_expr)
    }
}

fn while_loop(inter: &mut Interpreter, nodes: &[AstNode]) -> Result<WrappedType, RuntimeError> {
    if nodes.len() < 1 {
        return err!(ValueError, "Not enough arguments");
    }

    let condition = &nodes[0];
    let mut value = WrappedType::Null;

    while let WrappedType::Bool(true) = inter.eval(condition)? {
        value = block(inter, &nodes)?
    }

    return Ok(value);
}

lazy_static! {
    pub static ref SYMBOLS: HashMap<String, WrappedType> = {
        let mut h = HashMap::new();
        h.insert("set".into(), WrappedType::RustMacro(set));
        h.insert("list".into(), WrappedType::RustMacro(list));
        h.insert("block".into(), WrappedType::RustMacro(block));
        h.insert("if".into(), WrappedType::RustMacro(if_else));
        h.insert("while".into(), WrappedType::RustMacro(while_loop));
        h
    };
}
