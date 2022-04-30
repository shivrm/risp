use ::std::fmt;

mod lexer;
mod parser;
mod interpreter;
mod rispstd;

pub use self::lexer::Lexer;
pub use self::parser::Parser;
pub use self::interpreter::Intepreter;

pub struct Error {
    title: String,
    details: String
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.\n{}", self.title, self.details)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Number(i32),
    Name(String),
    OpenParen,
    CloseParen,
    EOF
}

#[derive(Clone)]
pub enum AstNode {
    Number(i32),
    Name(String),
    Expr(Vec<AstNode>)
}

#[derive(Clone)]
pub enum Type {
    Number(i32),
    List(Vec<Type>),
    BuiltinFn(&'static dyn Fn(Vec<Type>) -> Vec<Type>),
    Null
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Number(n) => write!(f, "{n}"),
            Type::BuiltinFn(_) => write!(f, "<Builtin Function>"),
            Type::List(elems) => {
                let mut iter = elems.iter();
                
                match iter.next() {
                    Some(el) => write!(f, "[{el}")?,
                    None => write!(f, "[")?
                }

                for el in iter {
                    write!(f, ", {el}")?;
                }

                write!(f, "]")
            }
            Type::Null => write!(f, "Null")
        }
    }
}

pub fn eval(text: &str) -> Result<Type, Error> {
    let lexer = Lexer::new(text);
    let mut parser = Parser::new(lexer)?;
    let ast = parser.parse_expr()?;
    let value = Intepreter::new().eval(ast);

    return Ok(value)
}