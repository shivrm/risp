use crate::risp::vm::{ErrorKind, Interpreter, RuntimeError, Value};
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

fn set(inter: &mut Interpreter, args: Vec<Value>) -> Result<Vec<Value>, RuntimeError> {
    if args.len() != 2 {
        return err!(
            ValueError,
            format!("expected 2 arguments, found {}", args.len())
        );
    }

    if let Value::Symbol(name) = &args[0] {
        let value = args[1].clone();
        inter.set_name(&name, value.clone());
        Ok(vec![value])
    } else {
        return err!(ValueError, format!("first argument must be a name"));
    }
}

fn block(inter: &mut Interpreter, nodes: &[AstNode]) -> Result<Value, RuntimeError> {
    let mut res = Value::Null;
    for node in nodes {
        res = inter.eval(&node)?;
    }

    Ok(res)
}

fn if_else(inter: &mut Interpreter, nodes: &[AstNode]) -> Result<Value, RuntimeError> {
    let has_else;
    match nodes.len() {
        2 => has_else = false,
        3 => has_else = true,
        _ => return err!(ValueError, "Incorrect number of arguments"),
    };

    let cond = &nodes[0];
    let if_expr = &nodes[1];

    if let Value::Bool(true) = inter.eval(cond)? {
        inter.eval(if_expr)
    } else {
        if has_else == false {
            return Ok(Value::Null);
        }

        let else_expr = &nodes[2];
        inter.eval(&else_expr)
    }
}

fn while_loop(inter: &mut Interpreter, nodes: &[AstNode]) -> Result<Value, RuntimeError> {
    if nodes.len() < 1 {
        return err!(ValueError, "Not enough arguments");
    }

    let condition = &nodes[0];
    let mut value = Value::Null;

    while let Value::Bool(true) = inter.eval(condition)? {
        value = block(inter, &nodes)?
    }

    return Ok(value);
}

lazy_static! {
    pub static ref SYMBOLS: HashMap<String, Value> = {
        let mut h = HashMap::new();
        h.insert("set".into(), Value::RustFn(set));
        h.insert("block".into(), Value::RustMacro(block));
        h.insert("if".into(), Value::RustMacro(if_else));
        h.insert("while".into(), Value::RustMacro(while_loop));
        h
    };
}
