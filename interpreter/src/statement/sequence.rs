use crate::{Environment, Run};
use ast::expression::{self, Expression};
use ast::statement::{Sequence, Statement};

impl Run for Sequence {
    fn run(&self, env: &mut Environment) -> Expression {
        for stmt in &self.sequence {
            match stmt {
                Statement::Return(_) => return stmt.run(env),
                _ => stmt.run(env),
            };
        }
        expression::nil()
    }
}
