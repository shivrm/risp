use crate::risp::Type;

/// Prints values to STDOUT, followed by a newline
pub fn println(_in: Vec<Type>) -> Vec<Type> {
    for el in _in.iter() {
        print!("{el} ");
    }

    print!("\n");

    return Vec::new()
}