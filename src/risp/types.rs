#[derive(Clone)]
pub enum Type {
    Number(i32),
    String(String),
    List(Vec<Type>),
    BuiltinFn(&'static dyn Fn(Vec<Type>) -> Vec<Type>),
    Null,
}

impl Type {
    pub fn repr(&self) -> String {
        match self {
            Type::Number(n) => n.to_string(),
            Type::String(s) => format!("\"{s:?}\""),

            Type::List(elems) =>  {
                let mut iter = elems.iter();
                let mut repr = String::from("[");

                match iter.next() {
                    Some(el) => repr += &el.repr(),
                    None => (())
                }

                for el in iter {
                    repr += ", ";
                    repr += &el.repr();
                }

                repr.push(']');
                repr
            }

            Type::BuiltinFn(_) => "<Builtin Function>".to_owned(),
            Type::Null => "".into()
        }
    }

    pub fn display(&self) -> String {
        match self {
            Type::String(s) => s.clone(),
            Type::Null => "Null".into(),
            _ => self.repr()
        }
    }
}