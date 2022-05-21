pub type Int = i32;
pub type Str = String;
pub type List = Vec<Type>;
pub type RustFn = fn(List) -> List;
pub struct Null;

#[derive(Clone)]
pub enum Type {
    Int(Int),
    Str(Str),
    List(List),
    RustFn(RustFn),
    Null,
}

pub trait RispPrint {
    fn repr(&self) -> String;
    fn display(&self) -> String;
}

// Function definitions could be done by a macro 🤔
impl RispPrint for Type {
    fn repr(&self) -> String {
        match self {
            Type::Int(n)    => n.repr(),
            Type::Str(s)    => s.repr(),
            Type::List(l)   => l.repr(),
            Type::RustFn(f) => f.repr(),
            Type::Null      => Null.repr(),
        }
    }

    fn display(&self) -> String {
        match self {
            Type::Int(n)    => n.display(),
            Type::Str(s)    => s.display(),
            Type::List(l)   => l.display(),
            Type::RustFn(f) => f.display(),
            Type::Null      => Null.display(),
        }
    }
}


impl RispPrint for Int {
    fn display(&self) -> String {
        self.to_string()
    }

    fn repr(&self) -> String {
        self.to_string()
    }
}

impl RispPrint for Str {
    fn display(&self) -> String {
        self.clone()
    }

    fn repr(&self) -> String {
        format!("{self:?}")
    }
}

impl RispPrint for List {
    fn repr(&self) -> String {
        let mut iter = self.iter();

        let mut result = match iter.next() {
            Some(el) => {
                let r = el.repr();
                format!(", {r}")
            },
            None => String::from("[")
        };

        for el in iter {
            result += &format!(", {}", el.repr())
        }

        result.push(']');

        result
    }

    fn display(&self) -> String {
        let mut iter = self.iter();

        let mut result = match iter.next() {
            Some(el) => format!("[{}", el.display()),
            None => String::from("[")
        };

        for el in iter {
            result += &format!(", {}", el.display())
        }

        result.push(']');

        result
    }
}

impl RispPrint for RustFn {
    fn display(&self) -> String {
        "<Rust Function>".to_owned()
    }

    fn repr(&self) -> String {
        "<Rust Function>".to_owned()
    }
}

impl RispPrint for Null {
    fn display(&self) -> String {
        "Null".to_owned()
    }

    fn repr(&self) -> String {
        String::new()
    }
}