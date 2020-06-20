pub mod expression;

use std::collections::HashMap;

type Environment = HashMap<String, ast::expression::Expression>;

pub trait Run {
    fn run(&self, env: &mut Environment) -> ast::expression::Expression;
}
