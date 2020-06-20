pub mod binary;
pub mod call;
pub mod constant;
pub mod function;
pub mod literal;

pub use binary::Binary;
pub use call::Call;
pub use constant::Constant;
pub use function::Function;
pub use literal::Literal;

#[derive(Clone, Debug)]
pub enum Expression {
    Binary(binary::Binary),
    Call(call::Call),
    Constant(constant::Constant),
    Function(function::Function),
    Literal(literal::Literal),
    Nil,
}

pub fn binary<L, O, R>(left: L, op: O, right: R) -> Expression
where
    L: Into<Expression>,
    O: Into<binary::Operator>,
    R: Into<Expression>,
{
    Expression::Binary(Binary {
        left: Box::new(left.into()),
        op: op.into(),
        right: Box::new(right.into()),
    })
}

pub fn call(name: impl Into<Literal>, params: Vec<Expression>) -> Expression {
    Expression::Call(Call {
        name: name.into(),
        params,
    })
}

pub fn constant(c: impl Into<Constant>) -> Expression {
    Expression::Constant(c.into())
}

pub fn function(
    name: Option<Literal>,
    params: Vec<impl Into<Literal>>,
    body: crate::statement::Sequence,
) -> Expression {
    Expression::Function(Function {
        name: name.map(|s| s.into()),
        params: params.into_iter().map(|s| s.into()).collect(),
        body,
    })
}

pub fn literal(s: impl Into<String>) -> Expression {
    Expression::Literal(Literal { value: s.into() })
}

pub fn nil() -> Expression {
    Expression::Nil
}

use Expression::*;

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Binary(b) => b.fmt(f),
            Call(c) => c.fmt(f),
            Constant(c) => c.fmt(f),
            Function(fun) => fun.fmt(f),
            Literal(s) => s.fmt(f),
            Nil => write!(f, "nil"),
        }
    }
}

impl From<binary::Binary> for Expression {
    fn from(b: binary::Binary) -> Self {
        Binary(b)
    }
}

impl From<constant::Constant> for Expression {
    fn from(c: constant::Constant) -> Self {
        Constant(c)
    }
}

impl From<literal::Literal> for Expression {
    fn from(l: literal::Literal) -> Self {
        Literal(l)
    }
}

impl From<function::Function> for Expression {
    fn from(f: function::Function) -> Self {
        Function(f)
    }
}
