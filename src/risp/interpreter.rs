use crate::risp::{AstNode, Type, Error};
use crate::risp::rispstd;

pub struct Intepreter { }

impl Intepreter {
    /// Create a new interpreter
    // This will be useful when interpreter will have default arguments
    // eg. symbol table
    pub fn new() -> Self {
        Intepreter { }
    }

    /// Gets the value associated with a name from the interpreter's 'symbol table'
    fn get_name(&self, name: String) -> Result<Type, Error> {
        if name == "println" {
            Ok(Type::BuiltinFn(&rispstd::println))
        } else {
            Err(Error::NameError(name.to_owned()))
        }
    }

    /// Evaluates an AST node
    pub fn eval(&self, node: AstNode) -> Result<Type, Error> {
        match node {
            AstNode::Name(name) => self.get_name(name.to_owned()),

            AstNode::Number(num) => Ok(Type::Number(num)),
            
            AstNode::String(s) => Ok(Type::String(s)),

            AstNode::Expr(mut nodes) => {
                // Expr has function as first argument and rest are params
                let func = self.eval(nodes.remove(0))?;
                
                let mut params = Vec::new();
                for node in nodes.iter() {
                    params.push(self.eval(node.clone())?);
                }
                
                // Make sure the function is a callable
                match func {
                    Type::BuiltinFn(f) => {
                        let mut result = f(params).clone();

                        // Return Null if nothing was returned, first element if
                        // only one was returned, and otherwise, a list
                        let value = match result.len() {
                            0 => Type::Null,
                            1 => result.pop().unwrap(),
                            _ => Type::List(result)
                        };

                        Ok(value)
                    },
                    _ => Err(Error::CallError(format!("{func}")))
                }
            }
        }
    }
}