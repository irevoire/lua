use nom::bytes::streaming::{is_not, tag, take_until, take_while1};
use nom::character::is_alphanumeric;
use nom::character::streaming::{char, digit1};
use nom::number::streaming::double;
use nom::sequence::delimited;
use nom::{error::ErrorKind, Err, IResult};

#[derive(Debug, PartialEq)]
pub enum Lexeme {
    // == keywords ==
    Break,
    Do,
    Else,
    Elseif,
    End,
    False,
    For,
    Function,
    Goto,
    If,
    In,
    Local,
    Nil,
    Not,
    Or,
    Repeat,
    Return,
    Then,
    True,
    Until,
    While,

    // symbol
    Plus,
    Minus,
    Mul,
    Div,
    Idiv,
    Mod,
    Concat,
    Length,
    Exponentiation,
    UnaryNot,
    BinaryAnd,
    BinaryOr,
    RightShift,
    LeftShift,
    OpenParenthesis,
    CloseParenthesis,
    Equality,
    Inequality,
    LessThan,
    MoreThan,
    LessOrEqual,
    MoreOrEqual,
    Assign,

    // separator
    SemiColon,
    Dot,
    NewLine,

    // comments
    Comments,

    // == user defined value ==
    Int(i64),
    Float(f64),
    String(String),
    Identifier(String),
}

impl std::fmt::Display for Lexeme {
    fn fmt(&self, form: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lexeme::Int(v) => write!(form, "{}", v),
            Lexeme::Float(v) => write!(form, "{}", v),
            Lexeme::String(v) => write!(form, "{}", v),
            Lexeme::Identifier(v) => write!(form, "{}", v),
            o => write!(form, "{:?}", o),
        }
    }
}

macro_rules! define_keyword {
    ($fn:ident, $lex:ident, $tag:tt) => {
        pub fn $fn(input: &[u8]) -> IResult<&[u8], Lexeme> {
            let (remaining, _) = tag($tag)(input)?;
            Ok((remaining, Lexeme::$lex))
        }
    };
}

// Keywords
define_keyword!(parse_break, Break, "break");
define_keyword!(parse_do, Do, "do");
define_keyword!(parse_else, Else, "else");
define_keyword!(parse_elseif, Elseif, "elseif");
define_keyword!(parse_end, End, "end");
define_keyword!(parse_false, False, "false");
define_keyword!(parse_for, For, "for");
define_keyword!(parse_function, Function, "function");
define_keyword!(parse_goto, Goto, "goto");
define_keyword!(parse_if, If, "if");
define_keyword!(parse_in, In, "in");
define_keyword!(parse_local, Local, "local");
define_keyword!(parse_nil, Nil, "nil");
define_keyword!(parse_not, Not, "not");
define_keyword!(parse_or, Or, "Or");
define_keyword!(parse_repeat, Repeat, "repeat");
define_keyword!(parse_return, Return, "return");
define_keyword!(parse_then, Then, "then");
define_keyword!(parse_true, True, "true");
define_keyword!(parse_until, Until, "until");
define_keyword!(parse_while, While, "while");

// Symbol
define_keyword!(parse_plus, Plus, "+");
define_keyword!(parse_minus, Minus, "-");
define_keyword!(parse_mul, Mul, "*");
define_keyword!(parse_div, Div, "/");
define_keyword!(parse_idiv, Idiv, "//");
define_keyword!(parse_mod, Mod, "%");
define_keyword!(parse_concat, Concat, "..");
define_keyword!(parse_length, Length, "#");
define_keyword!(parse_exponentiation, Exponentiation, "^");
define_keyword!(parse_unary_not, UnaryNot, "!");
define_keyword!(parse_binary_and, BinaryAnd, "&");
define_keyword!(parse_binary_or, BinaryOr, "|");
define_keyword!(parse_right_shift, RightShift, ">>");
define_keyword!(parse_left_shift, LeftShift, "<<");
define_keyword!(parse_open_parenthesis, OpenParenthesis, "(");
define_keyword!(parse_close_parenthesis, CloseParenthesis, ")");

// comparison
define_keyword!(parse_equality, Equality, "==");
define_keyword!(parse_inequality, Inequality, "~=");
define_keyword!(parse_less_than, LessThan, "<");
define_keyword!(parse_more_than, MoreThan, ">");
define_keyword!(parse_less_or_equal, LessOrEqual, "<=");
define_keyword!(parse_more_or_equal, MoreOrEqual, ">=");

// assignment
define_keyword!(parse_assign, Assign, "=");

// separator
define_keyword!(parse_semi_colon, SemiColon, ";");
define_keyword!(parse_dot, Dot, ".");

pub fn parse_comment(input: &[u8]) -> IResult<&[u8], Lexeme> {
    let (remaining, _) = tag("--")(input)?;
    let (remaining, _) = take_until("\n")(remaining)?;

    // remove the \n we matched in the take_until
    Ok((&remaining[1..], Lexeme::Comments))
}

pub fn parse_int(input: &[u8]) -> IResult<&[u8], Lexeme> {
    let (remaining, result) = digit1(input)?;
    if let Ok(string) = std::str::from_utf8(result) {
        if let Ok(i) = i64::from_str_radix(string, 10) {
            return Ok((remaining, Lexeme::Int(i)));
        }
    }
    Err(Err::Error((input, ErrorKind::Digit)))
}

pub fn parse_float(input: &[u8]) -> IResult<&[u8], Lexeme> {
    let (remaining, result) = double(input)?;
    Ok((remaining, Lexeme::Float(result)))
}

pub fn parse_string(input: &[u8]) -> IResult<&[u8], Lexeme> {
    let (remaining, result) = delimited(char('"'), is_not("\""), char('"'))(input)?;
    if let Ok(s) = std::str::from_utf8(result) {
        return Ok((remaining, Lexeme::String(String::from(s))));
    }
    Err(Err::Error((input, ErrorKind::Char)))
}

pub fn is_ident(chr: u8) -> bool {
    is_alphanumeric(chr) || chr == b'_'
}

pub fn parse_identifier(input: &[u8]) -> IResult<&[u8], Lexeme> {
    let (remaining, result) = take_while1(is_ident)(input)?;
    if let Ok(s) = std::str::from_utf8(result) {
        return Ok((remaining, Lexeme::Identifier(String::from(s))));
    }
    Err(Err::Error((input, ErrorKind::Char)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::Needed;
    use nom::Needed::Unknown;

    #[test]
    fn test_parse_break() {
        let test = b"break";
        assert_eq!(parse_break(test), Ok((&b""[..], Lexeme::Break)));
        let test = b"breaky";
        assert_eq!(parse_break(test), Ok((&b"y"[..], Lexeme::Break)));
        let test = b"br";
        assert_eq!(parse_break(test), Err(Err::Incomplete(Needed::Size(5))));
        let test = b"prout";
        assert_eq!(
            parse_break(test),
            Err(Err::Error((&b"prout"[..], ErrorKind::Tag)))
        );
    }

    #[test]
    fn test_parse_plus() {
        let test = b"+";
        assert_eq!(parse_plus(test), Ok((&b""[..], Lexeme::Plus)));
        let test = b"+=";
        assert_eq!(parse_plus(test), Ok((&b"="[..], Lexeme::Plus)));
        let test = b"prout";
        assert_eq!(
            parse_plus(test),
            Err(Err::Error((&b"prout"[..], ErrorKind::Tag)))
        );
    }

    #[test]
    fn test_parse_comment() {
        let test = b"--\n";
        assert_eq!(parse_comment(test), Ok((&b""[..], Lexeme::Comments)));
        let test = b"-- the comment \nlala";
        assert_eq!(parse_comment(test), Ok((&b"lala"[..], Lexeme::Comments)));
        let test = b"prout";
        assert_eq!(
            parse_comment(test),
            Err(Err::Error((&b"prout"[..], ErrorKind::Tag)))
        );
    }

    #[test]
    fn test_parse_int() {
        let test = b"42 ";
        assert_eq!(parse_int(test), Ok((&b" "[..], Lexeme::Int(42))));
        let test = b"42lala";
        assert_eq!(parse_int(test), Ok((&b"lala"[..], Lexeme::Int(42))));
        let test = b"42"; // the integer may be longer than that
        assert_eq!(parse_int(test), Err(Err::Incomplete(Needed::Size(1))));
        let test = b"prout";
        assert_eq!(
            parse_int(test),
            Err(Err::Error((&b"prout"[..], ErrorKind::Digit)))
        );
    }

    #[test]
    fn test_parse_float() {
        let test = b"42 ";
        assert_eq!(parse_float(test), Ok((&b" "[..], Lexeme::Float(42.0))));
        let test = b"42.34lala";
        assert_eq!(parse_float(test), Ok((&b"lala"[..], Lexeme::Float(42.34))));
        let test = b"42. "; // the integer may be longer than that
        assert_eq!(parse_float(test), Ok((&b" "[..], Lexeme::Float(42.0))));
        let test = b"42"; // the integer may be longer than that
        assert_eq!(parse_float(test), Err(Err::Incomplete(Unknown)));
        let test = b"42."; // the integer may be longer than that
        assert_eq!(parse_float(test), Err(Err::Incomplete(Unknown)));
        let test = b"prout";
        assert_eq!(
            parse_float(test),
            Err(Err::Error((&b"prout"[..], ErrorKind::Float)))
        );
    }

    #[test]
    fn test_parse_string() {
        let test = b"\"bonjour\"";
        assert_eq!(
            parse_string(test),
            Ok((&b""[..], Lexeme::String(String::from("bonjour"))))
        );
        let test = b"\"bonjour\"lala";
        assert_eq!(
            parse_string(test),
            Ok((&b"lala"[..], Lexeme::String(String::from("bonjour"))))
        );
        let test = b"\"bonjour la Fra";
        assert_eq!(parse_string(test), Err(Err::Incomplete(Needed::Size(1))));
        let test = b"prout ";
        assert_eq!(
            parse_string(test),
            Err(Err::Error((&b"prout "[..], ErrorKind::Char)))
        );
        let test = b"prout";
        assert_eq!(
            parse_string(test),
            Err(Err::Error((&b"prout"[..], ErrorKind::Char)))
        );
    }

    #[test]
    fn test_is_alphanumeric() {
        assert!(!is_alphanumeric(b'\"'));
    }

    #[test]
    fn test_parse_identifier() {
        let test = b"bonjour ";
        assert_eq!(
            parse_identifier(test),
            Ok((&b" "[..], Lexeme::Identifier(String::from("bonjour"))))
        );
        let test = b"__add4 prout";
        assert_eq!(
            parse_identifier(test),
            Ok((&b" prout"[..], Lexeme::Identifier(String::from("__add4"))))
        );
        let test = b"bonjour";
        assert_eq!(
            parse_identifier(test),
            Err(Err::Incomplete(Needed::Size(1)))
        );
        let test = b" prout";
        assert_eq!(
            parse_identifier(test),
            Err(Err::Error((&b" prout"[..], ErrorKind::TakeWhile1)))
        );
    }
}
