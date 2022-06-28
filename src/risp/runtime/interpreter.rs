use std::collections::HashMap;

use super::{rispstd, macros, ErrorKind, RuntimeError, RispType, Type};
use crate::risp::{AstNode, shared::Op};

macro_rules! err {
    ($kind:ident, $msg:expr) => {
        Err(RuntimeError {
            kind: ErrorKind::$kind,
            msg: $msg.into()
        })
    };
}

// Operator functions have this type signature
type OpFn = fn(&Type, &Type) -> Option<Type>;

/// Interprets ASTs
pub struct Interpreter {
    frame: HashMap<String, Type>
}

impl Interpreter {
    /// Creates a new interpreter.
    ///
    /// Currently, this does not do much. Once a prelude is added, this
    /// function can be used to initialize it.
    pub fn new() -> Self {
        let default_frame: HashMap<String, Type> = {
            let mut h = HashMap::new();
            h.extend(rispstd::SYMBOLS.clone().into_iter());
            h.extend(macros::SYMBOLS.clone().into_iter());
            h.insert("true".into(), Type::Bool(true));
            h.insert("false".into(), Type::Bool(false));
            h
        };

        Self { frame: default_frame }
    }

    /// Gets the value associated with a name from the interpreter's 'symbol table'
    /// Currently, this just gets them from the SYMBOLS HashMap in the standard library.
    pub fn get_name(&self, name: &str) -> Result<Type, RuntimeError> {
        match self.frame.get(name) {
            Some(value) => Ok(value.clone()),
            None => err!(NameError, format!("{name} is not defined")),
        }
    }

    pub fn set_name(&mut self, name: &str, value: Type) {
        self.frame.insert(name.into(), value);
    }

    /// Calls a function that's implemented in Rust. The function must accept a `Vec<Type>` as an argument
    /// and return a `Vec<Type>`.
    pub fn call_rustfn(
        &self,
        func: fn(Vec<Type>) -> Result<Vec<Type>, RuntimeError>,
        params: Vec<Type>,
    ) -> Result<Type, RuntimeError> {
        let result = func(params)?;

        // Returns Null if the function returns an empty Vec.
        // If the Vec contains one value, returns the value
        // If the Vec contains more than one value, returns it as a list
        let result = match result.len() {
            0 => Type::Null,
            1 => result[0].clone(),
            _ => Type::List(result),
        };

        Ok(result)
    }

    /// Calls an operator `op` on a Vec containing operands.
    ///
    /// This operator is evaluated similar to a `.reduce()`.
    /// i.e., `(+ a b c d)` will be evaluated as `((a + b) + c) + d`
    pub fn call_operator(&self, op: Op, operands: Vec<Type>) -> Result<Type, RuntimeError> {
        // a + b can be evaluated as `a.add(b)` or as `b.radd(a)`. This is useful when
        // a does not directly implement `add` for b.
        // The interpreter always tries to use the primary fn first. If this is not implemented,
        // then it uses the alternate fn.
        let (primary_fn, alternate_fn) = match op {
            Op::Plus => (RispType::add as OpFn, RispType::radd as OpFn),
            Op::Minus => (RispType::sub as OpFn, RispType::rsub as OpFn),
            Op::Star => (RispType::mul as OpFn, RispType::rmul as OpFn),
            Op::Slash => (RispType::div as OpFn, RispType::rdiv as OpFn),
            Op::Equal => (RispType::equal as OpFn, RispType::equal as OpFn),
            Op::Greater => (RispType::greater as OpFn, RispType::less as OpFn),
            Op::Less => (RispType::less as OpFn, RispType::greater as OpFn)
        };

        let mut params = operands.iter();

        // Stores the left operand for each application of the operator.
        let mut left = match params.next() {
            Some(v) => v.clone(),
            None => return err!(TypeError, "expected at least 1 argument"),
        };

        for right in params {
            // Tries to use the primary function
            left = match primary_fn(&left, &right) {
                Some(v) => v,
                // Use secondary function only if primary function fails
                None => match alternate_fn(right, &left) {
                    Some(v) => v,

                    // If both fail, return an error
                    None => {
                        let error_msg = format!("invalid operand types for {}: {} and {}", op.display(), left.type_name(), right.type_name());
                        return err!(TypeError, error_msg)
                    },
                },
            };
        }

        Ok(left)
    }

    /// Evaluates an AST node
    pub fn eval(&mut self, node: AstNode) -> Result<Type, RuntimeError> {
        match node {
            AstNode::Name(name) => self.get_name(&name),

            // These just involve transposing the value from an AstNode to a Type
            AstNode::Int(num) => Ok(Type::Int(num)),
            AstNode::Float(f) => Ok(Type::Float(f)),
            AstNode::Str(s) => Ok(Type::Str(s)),
            AstNode::Operator(op) => Ok(Type::Operator(op)),

            AstNode::Expr(mut nodes) => {
                if nodes.is_empty() {
                    return err!(ValueError, "expression is empty");
                };
                
                // Expr has function as first argument and rest are parameters
                let func = nodes.remove(0);
                let func = self.eval(func)?;

                if let Type::RustMacro(mac) = func {
                    return Ok(mac(self, nodes)?);
                }

                // Evaluate each parameter
                let mut params = Vec::new();
                for node in nodes.iter() {
                    params.push(self.eval(node.clone())?);
                }

                // Make sure the function is a callable
                match func {
                    Type::RustFn(f) => self.call_rustfn(f, params),

                    Type::Operator(op) => self.call_operator(op, params),

                    _ => err!(TypeError, format!("{} is not callable", func.type_name())),
                }
            }
        }
    }
}
