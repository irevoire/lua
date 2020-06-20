use ast::expression::{binary, constant, literal, Constant};
use ast::statement::{assignment, Sequence, Statement};

fn main() {
    let left = constant(Constant::Int(1));
    let right = constant(Constant::Int(2));

    let expr = binary(left, binary::Operator::Add, right);

    let var = literal("a");
    let stmt = assignment(var, expr);

    println!("{}", stmt);
}
