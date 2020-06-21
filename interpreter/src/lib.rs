#![feature(box_patterns)]

pub mod environment;
pub mod expression;
pub mod statement;

pub use environment::Environment;

pub trait Run {
    fn run(&self, env: &mut Environment) -> ast::expression::Expression;
}
