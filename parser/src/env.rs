use crate::function::Function;
use crate::value::Value;
use std::collections::HashMap;

pub struct Env {
    pub functions: HashMap<String, Function>,
    pub vars: HashMap<String, Value>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            functions: HashMap::new(),
            vars: HashMap::new(),
        }
    }

    pub fn base() -> Self {
        let mut functions = HashMap::new();
        functions.insert(String::from("print"), Function::new());

        Env {
            functions,
            vars: HashMap::new(),
        }
    }
}
