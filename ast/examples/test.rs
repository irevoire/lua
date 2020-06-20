use ast::expression::{binary, literal::Literal, Expression};
fn main() {
    let left = Expression::Literal(Literal::Int(1));
    let right = Expression::Literal(Literal::Int(2));

    let expr = Expression::Binary(binary::Binary {
        left: Box::new(left),
        op: binary::Operator::Add,
        right: Box::new(right),
    });

    println!("{}", expr);
}
