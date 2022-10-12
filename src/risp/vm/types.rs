use crate::{AstNode, risp::{Op, ErrorKind}};
use super::{Interpreter, RuntimeError};

#[derive(Clone)]
pub struct RispFn {
    params: Vec<String>,
    body: AstNode
}

impl RispFn {
    pub fn new(params: Vec<String>, body: AstNode) -> Self {
        RispFn { params, body: body }
    }
}

#[derive(Clone)]
pub enum Value {
    Int(i32),
    Bool(bool),
    Float(f64),
    Str(String),
    List(Vec<Value>),
    RustFn(fn (&mut Interpreter, Vec<Value>) -> Result<Vec<Value>, RuntimeError>),
    RustMacro(fn (&mut Interpreter, &[AstNode]) -> Result<Value, RuntimeError>),
    RispFn(RispFn),
    Operator(Op),
    Symbol(String),
    Null,
}

use Value::*;

impl Value {
    pub fn type_name(&self) -> String {
        match self {
            Int(_) => "int".into(),
            Bool(_) => "bool".into(),
            Float(_) => "float".into(),
            Str(_) => "str".into(),
            List(_) => "list".into(),
            RustFn(_) => "rustfn".into(),
            RispFn(_) => "rispfn".into(),
            RustMacro(_) => "rustmacro".into(),
            Operator(_) => "operator".into(),
            Symbol(_) => "symbol".into(),
            Null => "null".into()
        }
    }

    pub fn repr(&self) -> String {
        match self {
            Int(a) => a.to_string(),
            Bool(a) => a.to_string(),
            Float(a) => a.to_string(),
            Str(a) => a.to_string(),
            List(_) => "[]".into(),
            RustFn(_) => "<Rust Function>".into(),
            RustMacro(_) => "<Rust Macro>".into(),
            RispFn(_) => "<Risp Function>".into(),
            Operator(a) => match a {
                Op::Plus => "+".into(),
                Op::Minus => "-".into(),
                Op::Star => "*".into(),
                Op::Slash => "/".into(),
                Op::Equal => "=".into(),
                Op::Greater => ">".into(),
                Op::Less => "<".into()
            },
            Symbol(s) => format!("<Symbol {s}>"),
            Null => "null".into()
        }
    }

    pub fn display(&self) -> String {
        self.repr()
    }

    pub fn oneside_binary_op(&self, rhs: &Value, op: &Op) -> Result<Value, RuntimeError> {
        macro_rules! impl_default {
            ($a:expr, $b:expr, $out:ident) => {{
                use Op::*;
                let value = match op {
                    Plus => $out($a + $b),
                    Minus => $out($a - $b),
                    Star => $out($a * $b),
                    Slash => $out($a / $b),
                    Equal => Bool($a == $b),
                    Greater => Bool($a > $b),
                    Less => Bool($a < $b),
                };
                return Ok(value)
            }}
        }
        
        match (self, rhs) {
            (Int(a), Int(b)) => impl_default!(a, b, Int),
            (Bool(a), Int(b)) => impl_default!(&(*a as i32), b, Int),
            (Float(a), Int(b)) => impl_default!(a, &(*b as f64), Float),
            (Str(a), Str(b)) => {
                if op == &Op::Plus {
                    return Ok(Str(a.clone() + b))
                }
            }
            (Str(a), Int(b)) => {
                if op == &Op::Star {
                    return Ok(Str(a.repeat(*b as usize)))
                }
            },
            _ => (),
        }

        return Err(RuntimeError {
            kind: ErrorKind::TypeError,
            msg: "".into()
        })
        
    }

    pub fn binary_op(&self, rhs: &Value, op: &Op) -> Result<Value, RuntimeError> {
        return match self.oneside_binary_op(rhs, op) {
            Err(_) => rhs.oneside_binary_op(self, op),
            ok => ok
        }
    }
}