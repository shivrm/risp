use crate::risp::{AstNode, Type};

pub struct Intepreter {
}

impl Intepreter {
    pub fn new() -> Self {
        Intepreter {}
    }

    fn get_name(&self, name: String) -> Type {
        todo!();
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
                    Type::BuiltinFn(f) => f(params)[0].clone(),
                    _ => panic!("function is not callable")
                }
            }
        }
    }
}