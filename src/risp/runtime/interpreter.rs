use std::collections::HashMap;

use super::{macros, rispstd, ErrorKind, RuntimeError, Type, WrappedType};
use crate::risp::{shared::Op, AstNode};

macro_rules! err {
    ($kind:ident, $msg:expr) => {
        Err(RuntimeError {
            kind: ErrorKind::$kind,
            msg: $msg.into(),
        })
    };
}

// Operator functions have this type signature
type CmpOpFn = fn(&WrappedType, &WrappedType) -> Option<WrappedType>;
type BinOpFn = fn(&WrappedType, &WrappedType) -> Option<bool>;

/// Interprets ASTs
pub struct Interpreter {
    frame: HashMap<String, WrappedType>,
}

impl Interpreter {
    /// Creates a new interpreter.
    pub fn new() -> Self {
        let default_frame: HashMap<String, WrappedType> = {
            let mut h = HashMap::new();
            h.extend(rispstd::SYMBOLS.clone().into_iter());
            h.extend(macros::SYMBOLS.clone().into_iter());
            h.insert("true".into(), WrappedType::Bool(true));
            h.insert("false".into(), WrappedType::Bool(false));
            h
        };

        Self {
            frame: default_frame,
        }
    }

    /// Gets the value associated with a name from the interpreter's 'symbol table'
    /// Currently, this just gets them from the SYMBOLS HashMap in the standard library.
    pub fn get_name(&self, name: &str) -> Result<WrappedType, RuntimeError> {
        match self.frame.get(name) {
            Some(value) => Ok(value.clone()),
            None => err!(NameError, format!("{name} is not defined")),
        }
    }

    pub fn set_name(&mut self, name: &str, value: WrappedType) {
        self.frame.insert(name.into(), value);
    }

    /// Calls a function that's implemented in Rust. The function must accept a `Vec<Type>` as an argument
    /// and return a `Vec<Type>`.
    pub fn call_rustfn(
        &self,
        func: fn(Vec<WrappedType>) -> Result<Vec<WrappedType>, RuntimeError>,
        params: Vec<WrappedType>,
    ) -> Result<WrappedType, RuntimeError> {
        let result = func(params)?;

        // Returns Null if the function returns an empty Vec.
        // If the Vec contains one value, returns the value
        // If the Vec contains more than one value, returns it as a list
        let result = match result.len() {
            0 => WrappedType::Null,
            1 => result[0].clone(),
            _ => WrappedType::List(result),
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
        operands: Vec<WrappedType>,
    ) -> Result<WrappedType, RuntimeError> {
        // a + b can be evaluated as `a.add(b)` or as `b.radd(a)`. This is useful when
        // a does not directly implement `add` for b.
        // The interpreter always tries to use the primary fn first. If this is not implemented,
        // then it uses the alternate fn.
        let (primary_fn, alternate_fn) = match op {
            Op::Plus => (Type::add as CmpOpFn, Type::radd as CmpOpFn),
            Op::Minus => (Type::sub as CmpOpFn, Type::rsub as CmpOpFn),
            Op::Star => (Type::mul as CmpOpFn, Type::rmul as CmpOpFn),
            Op::Slash => (Type::div as CmpOpFn, Type::rdiv as CmpOpFn),
            _ => return self.call_boolean_op(op, operands),
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
                        let error_msg = format!(
                            "invalid operand types for {}: {} and {}",
                            op.display(),
                            left.type_name(),
                            right.type_name()
                        );
                        return err!(TypeError, error_msg);
                    }
                },
            };
        }

        Ok(left)
    }

    pub fn call_boolean_op(
        &self,
        op: Op,
        operands: Vec<WrappedType>,
    ) -> Result<WrappedType, RuntimeError> {
        let (primary_fn, alternate_fn) = match op {
            Op::Equal => (Type::eq as BinOpFn, Type::eq as BinOpFn),
            Op::Greater => (Type::gt as BinOpFn, Type::lt as BinOpFn),
            Op::Less => (Type::lt as BinOpFn, Type::gt as BinOpFn),
            _ => unreachable!(),
        };

        let mut res = true;

        for window in operands.windows(2) {
            let left = &window[0];
            let right = &window[1];

            res = match primary_fn(&left, &right) {
                Some(v) => v,
                // Use secondary function only if primary function fails
                None => match alternate_fn(&right, &left) {
                    Some(v) => v,

                    // If both fail, return an error
                    None => {
                        let error_msg = format!(
                            "invalid operand types for {}: {} and {}",
                            op.display(),
                            left.type_name(),
                            right.type_name()
                        );
                        return err!(TypeError, error_msg);
                    }
                },
            };
        }

        Ok(res.into())
    }
    /// Evaluates an AST node
    pub fn eval(&mut self, node: &AstNode) -> Result<WrappedType, RuntimeError> {
        match node {
            AstNode::Name(name) => self.get_name(&name),

            // These just involve transposing the value from an AstNode to a Type
            AstNode::Int(num) => Ok(WrappedType::Int(*num)),
            AstNode::Float(f) => Ok(WrappedType::Float(*f)),
            AstNode::Str(s) => Ok(WrappedType::Str(s.clone())),
            AstNode::Operator(op) => Ok(WrappedType::Operator(*op)),

            AstNode::Expr(nodes) => {
                if nodes.is_empty() {
                    return err!(ValueError, "expression is empty");
                };

                // Expr has function as first argument and rest are parameters
                let func = &nodes[0];
                let func = self.eval(func)?;

                if let WrappedType::RustMacro(mac) = func {
                    return Ok(mac(self, &nodes[1..])?);
                }

                // Evaluate each parameter
                let mut params = Vec::new();
                for node in &nodes[1..] {
                    params.push(self.eval(node)?);
                }

                // Make sure the function is a callable
                match func {
                    WrappedType::RustFn(f) => self.call_rustfn(f, params),

                    WrappedType::Operator(op) => self.call_operator(op, params),

                    _ => err!(TypeError, format!("{} is not callable", func.type_name())),
                }
            }
        }
    }
}
