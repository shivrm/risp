use std::collections::HashMap;

use super::{macros, rispstd, ErrorKind, RuntimeError, Value};
use crate::risp::{shared::Op, AstNode};

macro_rules! err {
    ($kind:ident, $msg:expr) => {
        Err(RuntimeError {
            kind: ErrorKind::$kind,
            msg: $msg.into(),
        })
    };
}

/// Interprets ASTs
pub struct Interpreter {
    frame: HashMap<String, Value>,
}

impl Interpreter {
    /// Creates a new interpreter.
    pub fn new() -> Self {
        let default_frame: HashMap<String, Value> = {
            let mut h = HashMap::new();
            h.extend(rispstd::SYMBOLS.clone().into_iter());
            h.extend(macros::SYMBOLS.clone().into_iter());
            h.insert("true".into(), Value::Bool(true));
            h.insert("false".into(), Value::Bool(false));
            h
        };

        Self {
            frame: default_frame,
        }
    }

    /// Gets the value associated with a name from the interpreter's 'symbol table'
    /// Currently, this just gets them from the SYMBOLS HashMap in the standard library.
    pub fn get_name(&self, name: &str) -> Result<Value, RuntimeError> {
        match self.frame.get(name) {
            Some(value) => Ok(value.clone()),
            None => err!(NameError, format!("{name} is not defined")),
        }
    }

    pub fn set_name(&mut self, name: &str, value: Value) {
        self.frame.insert(name.into(), value);
    }

    /// Calls a function that's implemented in Rust. The function must accept a `Vec<Type>` as an argument
    /// and return a `Vec<Type>`.
    pub fn call_rustfn(
        &self,
        func: fn(Vec<Value>) -> Result<Vec<Value>, RuntimeError>,
        params: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let result = func(params)?;

        // Returns Null if the function returns an empty Vec.
        // If the Vec contains one value, returns the value
        // If the Vec contains more than one value, returns it as a list
        let result = match result.len() {
            0 => Value::Null,
            1 => result[0].clone(),
            _ => Value::List(result),
        };

        Ok(result)
    }

    /// Calls an operator `op` on a Vec containing operands.
    ///
    /// This operator is evaluated similar to a `.reduce()`.
    /// i.e., `(+ a b c d)` will be evaluated as `((a + b) + c) + d`
    pub fn call_operator(
        &self,
        op: Op,
        operands: Vec<Value>,
    ) -> Result<Value, RuntimeError> {

        if let Op::Equal | Op::Greater | Op::Less = op {
            return self.call_boolean_op(op, operands)
        }

        let mut params = operands.iter();

        // Stores the left operand for each application of the operator.
        let mut left = match params.next() {
            Some(v) => v.clone(),
            None => return err!(TypeError, "expected at least 1 argument"),
        };

        for right in params {
            left = left.binary_op(right, &op)?;
        }

        Ok(left)
    }

    pub fn call_boolean_op(
        &self,
        op: Op,
        operands: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let mut res = true;

        for window in operands.windows(2) {
            let left = &window[0];
            let right = &window[1];

            if let Value::Bool(b) = left.binary_op(right, &op)? {
                res = res && b;
            }
        }
        Ok(Value::Bool(res))
    }
    /// Evaluates an AST node
    pub fn eval(&mut self, node: &AstNode) -> Result<Value, RuntimeError> {
        match node {
            AstNode::Name(name) => self.get_name(&name),

            // These just involve transposing the value from an AstNode to a Type
            AstNode::Int(num) => Ok(Value::Int(*num)),
            AstNode::Float(f) => Ok(Value::Float(*f)),
            AstNode::Str(s) => Ok(Value::Str(s.clone())),
            AstNode::Operator(op) => Ok(Value::Operator(*op)),

            AstNode::Expr(nodes) => {
                if nodes.is_empty() {
                    return err!(ValueError, "expression is empty");
                };

                // Expr has function as first argument and rest are parameters
                let func = &nodes[0];
                let func = self.eval(func)?;

                if let Value::RustMacro(mac) = func {
                    return Ok(mac(self, &nodes[1..])?);
                }

                // Evaluate each parameter
                let mut params = Vec::new();
                for node in &nodes[1..] {
                    params.push(self.eval(node)?);
                }

                // Make sure the function is a callable
                match func {
                    Value::RustFn(f) => self.call_rustfn(f, params),

                    Value::Operator(op) => self.call_operator(op, params),

                    _ => err!(TypeError, format!("{} is not callable", func.type_name())),
                }
            }
        }
    }
}
