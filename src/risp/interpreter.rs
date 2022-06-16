use crate::risp::{AstNode, Error, Type, RispType, Op, rispstd};

type OpFn = fn(&Type, &Type) -> Option<Type>;

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
            Some(value) => Ok(value.clone()),
            None => Err(Error::NameError(name))
        }
    }

    /// Evaluates an AST node
    pub fn eval(&self, node: AstNode) -> Result<Type, Error> {
        match node {
            AstNode::Name(name) => self.get_name(name.to_owned()),

            AstNode::Integer(num) => Ok(Type::Int(num)),

            AstNode::Float(f) => Ok(Type::Float(f)),

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
                        let (fun, alternate): (OpFn, OpFn) = match op {
                            Op::Plus => (RispType::add as OpFn, RispType::radd as OpFn),
                            Op::Minus => (RispType::sub as OpFn, RispType::rsub as OpFn),
                            Op::Star => (RispType::mul as OpFn, RispType::rmul as OpFn),
                            Op::Slash => (RispType::div as OpFn, RispType::rdiv as OpFn),
                        };

                        let mut params = params.iter();
                        let mut left = match params.next() {
                            Some(v) => v.clone(),
                            None => return Err(Error::Error("Not enough operands for operator".into()))
                        };

                        for param in params {
                            left = match fun(&left, &param) {
                                Some(v) => v,
                                None => match alternate(param, &left) {
                                    Some(v) => v,
                                    None => return Err(Error::OpError(left.repr(), op.repr(), param.repr()))
                                }
                            };
                        }

                        Ok(left)
                    }

                    _ => Err(Error::CallError(format!("{}", func.display()))),
                }
            }
        }
    }
}
