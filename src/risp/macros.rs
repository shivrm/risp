use std::collections::HashMap;
use crate::risp::Type;

lazy_static! {
    pub static ref SYMBOLS: HashMap<&'static str, Type> = {
        let h = HashMap::new();
        h
    };
}
