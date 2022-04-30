use crate::risp::{AstNode, Type, Error};
use crate::risp::rispstd;

pub struct Intepreter {
}

impl Intepreter {
    pub fn new() -> Self {
        Intepreter {}
    }

    fn get_name(&self, name: String) -> Result<Type, Error> {
        if name == "println" {
            Ok(Type::BuiltinFn(&rispstd::println))
        } else {
            Err(Error {
                title: format!("Unknown name {name}"),
                details: "The interpreter could not find the name in the symbol table".to_owned()
            })
        }
    }

    pub fn eval(&self, node: AstNode) -> Result<Type, Error> {
        match node {
            AstNode::Name(name) => self.get_name(name),

            AstNode::Number(num) => Ok(Type::Number(num)),
            
            AstNode::Expr(mut nodes) => {
                let func = self.eval(nodes.remove(0))?;
                
                let mut params = Vec::new();
                for node in nodes.iter() {
                    params.push(self.eval(node.clone())?);
                }
                
                match func {
                    Type::BuiltinFn(f) => {
                        let mut result = f(params).clone();

                        let value = match result.len() {
                            0 => Type::Null,
                            1 => result.pop().unwrap(),
                            _ => Type::List(result)
                        };

                        Ok(value)
                    },
                    _ => Err(Error {
                        title: "Uncallable function".to_owned(),
                        details: format!("{func} is not a callable function")
                    })
                }
            }
        }
    }
}