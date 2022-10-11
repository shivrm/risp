use std::collections::HashMap;

use super::{ErrorKind, RuntimeError, Value};
use crate::risp::{shared::Op, AstNode, stdlib};

/// Used for conveniently creating [`RuntimeError`]s
macro_rules! err {
    ($kind:ident, $msg:expr) => {
        Err(RuntimeError {
            kind: ErrorKind::$kind,
            msg: $msg.into(),
        })
    };
}

/// A struct that interprets ASTs
pub struct Interpreter {
    frame: HashMap<String, Value>,
}

impl Interpreter {
    /// Creates a new interpreter.
    pub fn new() -> Self {
        // Create the interpreter's symbol table
        let default_frame: HashMap<String, Value> = {
            let mut h = HashMap::new();
            h.extend(stdlib::functions::SYMBOLS.clone().into_iter());
            h.extend(stdlib::macros::SYMBOLS.clone().into_iter());
            h.insert("true".into(), Value::Bool(true));
            h.insert("false".into(), Value::Bool(false));
            h
        };

        Self {
            frame: default_frame,
        }
    }

    /// Retrieves the [`Value`] associated with a name in the interpreter's
    /// symbol table. Returns a [`RuntimeError`] if the name is not present
    /// in the symbol table.
    pub fn get_name(&self, name: &str) -> Result<Value, RuntimeError> {
        match self.frame.get(name) {
            Some(value) => Ok(value.clone()),
            None => err!(NameError, format!("{name} is not defined")),
        }
    }

    /// Creates an entry in the interpreter's symbol table associating
    /// a name with a value. If an entry with the same name already
    /// exists, then its value is updated.
    pub fn set_name(&mut self, name: &str, value: Value) {
        self.frame.insert(name.into(), value);
    }

    /// Calls a native Rust function
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

    /// This method evaluates binary operators in a manner similar to
    /// `.reduce()`.
    /// 
    /// For example, `(+ 1 2 3 4)` is evaluated as `(+ (+ (+ 1 2) 3) 4)`
    /// which is `((1 + 2) + 3) + 4` in infix notation.
    /// 
    /// Handling of boolean operators is delegated to the `call_boolean_op`
    /// method.
    pub fn call_operator(
        &self,
        op: Op,
        operands: Vec<Value>,
    ) -> Result<Value, RuntimeError> {

        // Boolean operators need different chaining logic
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

    /// Evaluates boolean operators.
    /// 
    /// Boolean operators have different chaining rules compared to
    /// binary operators. `(< 1 2 3)` would be interpreted as
    /// `(1 < 2) and (2 < 3)`.
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
            // Names are evaluated by getting the value associated
            // with them.
            AstNode::Name(name) => self.get_name(&name),

            // Int, Float, Str, and Operator just involve transposing the
            // inner content into a Value
            AstNode::Int(num) => Ok(Value::Int(*num)),
            AstNode::Float(f) => Ok(Value::Float(*f)),
            AstNode::Str(s) => Ok(Value::Str(s.clone())),
            AstNode::Operator(op) => Ok(Value::Operator(*op)),

            // In expressions, the first item is the function to execute
            // And the rest of the items are the arguments
            AstNode::Expr(nodes) => {
                if nodes.is_empty() {
                    return err!(ValueError, "expression is empty");
                };

                // Moves the function into a seperate variable
                let func = &nodes[0];
                let func = self.eval(func)?;

                // Macros operate on AST nodes themselves, so they
                // can be called immedicately
                if let Value::RustMacro(mac) = func {
                    return mac(self, &nodes[1..]);
                }

                // Evaluate each argument
                let mut args = Vec::new();
                for node in &nodes[1..] {
                    args.push(self.eval(node)?);
                }

                // Make sure the function is a callable
                match func {
                    Value::RustFn(f) => self.call_rustfn(f, args),
                    Value::Operator(op) => self.call_operator(op, args),
                    _ => err!(TypeError, format!("{} is not callable", func.type_name())),
                }
            }
        }
    }
}
