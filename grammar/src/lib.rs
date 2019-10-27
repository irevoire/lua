pub use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    // == lexer internals ==
    #[end]
    ParseEnd,
    #[error]
    ParseError,

    // == keywords ==
    #[token = "and"]
    And,
    #[token = "break"]
    Break,
    #[token = "do"]
    Do,
    #[token = "else"]
    Else,
    #[token = "elseif"]
    Elseif,
    #[token = "end"]
    End,
    #[token = "false"]
    False,
    #[token = "for"]
    For,
    #[token = "function"]
    Function,
    #[token = "goto"]
    Goto,
    #[token = "if"]
    If,
    #[token = "in"]
    In,
    #[token = "local"]
    Local,
    #[token = "nil"]
    Nil,
    #[token = "not"]
    Not,
    #[token = "or"]
    Or,
    #[token = "repeat"]
    Repeat,
    #[token = "return"]
    Return,
    #[token = "then"]
    Then,
    #[token = "true"]
    True,
    #[token = "until"]
    Until,
    #[token = "while"]
    While,

    // operator
    #[token = "+"]
    Plus,
    #[token = "-"]
    Minus,
    #[token = "*"]
    Mul,
    #[token = "/"]
    Div,
    #[token = "%"]
    Mod,
    #[token = ".."]
    Concat,
    #[token = "#"]
    Length,
    #[token = "^"]
    Exponentiation,
    #[token = "!"]
    UnaryNot,
    #[token = "&"]
    BinaryAnd,
    #[token = "|"]
    BinaryOr,
    #[token = ">>"]
    RightShift,
    #[token = "<<"]
    LeftShift,
    #[token = "("]
    OpenParenthesis,
    #[token = ")"]
    CloseParenthesis,

    // comparison
    #[token = "=="]
    Equality,
    #[token = "~="]
    Inequality,
    #[token = "<"]
    LessThan,
    #[token = ">"]
    MoreThan,
    #[token = "<="]
    LessOrEqual,
    #[token = ">="]
    MoreOrEqual,
    #[token = "and"]
    LogicalAnd,
    #[token = "or"]
    LogicalOr,
    #[token = "not"]
    LogicalNot,

    // assignment
    #[token = "="]
    Assign,

    // separator
    #[token = ";"]
    SemiColon,

    // comments
    #[regex = "--.*"]
    Comments,

    // == user defined value ==
    #[regex = "[0-9]+"]
    Int,
    #[regex = "[0-9]+\\.[0-9]*"]
    Float,
    #[regex = "\"[^\"]*\""]
    String,
    #[regex = "[a-zA-Z][a-zA-Z0-9_\\-]*"]
    Var,
    #[regex = "\\."]
    Dot,
    #[regex = "\n"]
    NewLine,
}
