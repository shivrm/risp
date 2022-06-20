use std::collections::HashMap;
use crate::risp::{ Intepreter, Type, AstNode, ErrorKind };

fn set(inter: &mut Intepreter, mut nodes: Vec<AstNode>) -> Result<Type, ErrorKind> {
    if nodes.len() != 2 {
        return Err(ErrorKind::Error("Expected 2 arguments".into()))
    }

    if let AstNode::Name(name) = nodes.remove(0) {
        let value = inter.eval(nodes.pop().unwrap()).unwrap();
        inter.set_name(&name, value.clone());
        Ok(value)
    } else {
        Err(ErrorKind::Error("Expected first argument to be a name".into()))
    }
}

lazy_static! {
    pub static ref SYMBOLS: HashMap<String, Type> = {
        let mut h = HashMap::new();
        h.insert("set".into(), Type::RustMacro(set));
        h
    };
}
