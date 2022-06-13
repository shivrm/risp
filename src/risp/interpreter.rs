use crate::risp::{AstNode, Error, Type, RispType, Op, rispstd};

pub struct Intepreter {
}

impl Intepreter {
    /// Create a new interpreter
    // This will be useful when interpreter will have default arguments
    // eg. symbol table
    pub fn new() -> Self {
        Intepreter {}
    }

    /// Gets the value associated with a name from the interpreter's 'symbol table'
    fn get_name(&self, name: String) -> Result<Type, Error> {
        match rispstd::SYMBOLS.get(name.as_str()) {
            Some(f) => Ok(Type::RustFn(*f)),
            None => Err(Error::NameError(name))
        }
    }

    /// Evaluates an AST node
    pub fn eval(&self, node: AstNode) -> Result<Type, Error> {
        match node {
            AstNode::Name(name) => self.get_name(name.to_owned()),

            AstNode::Number(num) => Ok(Type::Int(num)),

            AstNode::String(s) => Ok(Type::Str(s)),

            AstNode::Operator(op) => Ok(Type::Operator(op)),

            AstNode::Expr(mut nodes) => {
                // Expr has function as first argument and rest are params
                let func = self.eval(nodes.remove(0))?;

                let mut params = Vec::new();
                for node in nodes.iter() {
                    params.push(self.eval(node.clone())?);
                }

                // Make sure the function is a callable
                match func {
                    Type::RustFn(f) => {
                        let mut result = f(params).clone();

                        // Return Null if nothing was returned, first element if
                        // only one was returned, and otherwise, a list
                        let value = match result.len() {
                            0 => Type::Null,
                            1 => result.pop().unwrap(),
                            _ => Type::List(result),
                        };

                        Ok(value)
                    }

                    Type::Operator(op) => {
                        let left = params.remove(0);
                        let right = params.remove(0);

                        let res = match op {
                            Op::Plus => left.add(&right),
                            Op::Minus => left.sub(&right),
                            Op::Star => left.mul(&right),
                            Op::Slash => left.div(&right)
                        };

                        if let Some(v) = res {
                            return Ok(v)
                        }

                        let res = match op {
                            Op::Plus => right.radd(&left),
                            Op::Minus => right.rsub(&left),
                            Op::Star => right.rmul(&left),
                            Op::Slash => right.rdiv(&left)
                        };

                        match res {
                            Some(v) => Ok(v),
                            None => Err(Error::OpError(left.display(), op.display(), right.display()))
                        }

                    }

                    _ => Err(Error::CallError(format!("{}", func.display()))),
                }
            }
        }
    }
}
