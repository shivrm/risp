use std::collections::HashMap;
use crate::risp::{ Intepreter, Type, AstNode };

fn set(inter: &mut Intepreter, mut nodes: Vec<AstNode>) -> Type {
    if nodes.len() != 2 {
        panic!("Wrong number of arguments for set (expected 2)");
    }

    if let AstNode::Name(name) = nodes.remove(0) {
        let value = inter.eval(nodes.pop().unwrap()).unwrap();
        inter.set_name(&name, value.clone());
        value
    } else {
        panic!("Expected first argument to be a name");
    }
}

lazy_static! {
    pub static ref SYMBOLS: HashMap<String, Type> = {
        let mut h = HashMap::new();
        h.insert("set".into(), Type::RustMacro(set));
        h
    };
}
