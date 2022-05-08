use crate::risp::Type;

/// Prints values to STDOUT, followed by a newline
pub fn println(_in: Vec<Type>) -> Vec<Type> {
    for el in _in.iter() {
        print!("{} ", el.display());
    }

    print!("\n");

    return Vec::new();
}


pub fn get_name(name: &str) -> Option<Type> {
    let value = match name {
        "println" => Type::BuiltinFn(&println),
        _ => return None
    };

    return Some(value)
}