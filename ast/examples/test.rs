use ast::expression::{binary, Constant, Expression};
fn main() {
    let left = Expression::Constant(Constant::Int(1));
    let right = Expression::Constant(Constant::Int(2));

    let expr = Expression::Binary(binary::Binary {
        left: Box::new(left),
        op: binary::Operator::Add,
        right: Box::new(right),
    });

    println!("{}", expr);
}
