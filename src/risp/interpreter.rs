use crate::risp::{AstNode, Error, Type, RispPrint };
extern crate libloading;

pub struct Intepreter {
    stdlib: libloading::Library
}

impl Intepreter {
    /// Create a new interpreter
    // This will be useful when interpreter will have default arguments
    // eg. symbol table
    pub fn new() -> Self {
        unsafe {
            Intepreter {
                stdlib: libloading::Library::new(
                    libloading::library_filename("lib/std")
                ).unwrap()
            }
        }
    }

    /// Gets the value associated with a name from the interpreter's 'symbol table'
    fn get_name(&self, name: String) -> Result<Type, Error> {
        unsafe {
            let symbol = match self.stdlib.get(name.as_bytes()) {
                Ok(s) => s,
                Err(_) => return Err(Error::NameError(name))
            };

            Ok(Type::RustFn(*symbol))
        }
    }

    /// Evaluates an AST node
    pub fn eval(&self, node: AstNode) -> Result<Type, Error> {
        match node {
            AstNode::Name(name) => self.get_name(name.to_owned()),

            AstNode::Number(num) => Ok(Type::Int(num)),

            AstNode::String(s) => Ok(Type::Str(s)),

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
                    _ => Err(Error::CallError(format!("{}", func.display()))),
                }
            }
        }
    }
}
