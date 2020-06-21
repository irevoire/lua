use ast::expression::{Expression, Literal};
use std::collections::HashMap;

type Env = HashMap<Literal, Expression>;

#[derive(Clone, Debug)]
pub struct Environment {
    stack: Vec<Env>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            stack: vec![Env::new()],
        }
    }

    pub fn local_insert(&mut self, lit: Literal, expr: Expression) {
        self.stack.last_mut().unwrap().insert(lit, expr);
    }

    pub fn global_insert(&mut self, lit: Literal, expr: Expression) {
        self.stack.first_mut().unwrap().insert(lit, expr);
    }

    pub fn get(&self, lit: &Literal) -> Option<&Expression> {
        self.stack
            .last()
            .unwrap()
            .get(lit)
            .or(self.stack.first().unwrap().get(lit))
    }

    pub fn push_scope(&mut self) {
        self.stack.push(Env::new())
    }

    pub fn pop_scope(&mut self) -> Env {
        self.stack
            .pop()
            .expect("trying to pop an environment while there is none remaining")
    }
}
