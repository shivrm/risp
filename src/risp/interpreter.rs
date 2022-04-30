use crate::risp::{AstNode, Type};
use crate::risp::rispstd;

pub struct Intepreter {
}

impl Intepreter {
    pub fn new() -> Self {
        Intepreter {}
    }

    fn get_name(&self, name: String) -> Type {
        if name == "println" {
            return Type::BuiltinFn(&rispstd::println)
        } else {
            panic!("Unknown function")
        }
    }

    pub fn eval(&self, node: AstNode) -> Type {
        match node {
            AstNode::Name(name) => self.get_name(name),
            AstNode::Number(num) => Type::Number(num),
            AstNode::Expr(mut nodes) => {
                let func = self.eval(nodes.remove(0));
                let params: Vec<Type> = nodes
                                .iter()
                                .map(|n| self.eval(n.clone()))
                                .collect();
                
                match func {
                    Type::BuiltinFn(f) => {
                        let mut result = f(params).clone();

                        match result.len() {
                            0 => Type::Null,
                            1 => result.pop().unwrap(),
                            _ => Type::List(result)
                        }
                    },
                    _ => panic!("function is not callable")
                }
            }
        }
    }
}