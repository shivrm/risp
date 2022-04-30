use crate::risp::Type;

fn println(_in: Vec<Type>) -> Vec<Type> {
    for el in _in.iter() {
        print!("{el} ");
    }

    print!("\n");

    return Vec::new()
}