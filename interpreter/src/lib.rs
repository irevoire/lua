pub mod expression;
pub mod statement;

use std::collections::HashMap;

type Environment = HashMap<ast::expression::Literal, ast::expression::Expression>;

pub trait Run {
    fn run(&self, env: &mut Environment) -> ast::expression::Expression;
}
