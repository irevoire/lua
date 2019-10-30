use crate::lexer::*;
use nom::branch::alt;
use nom::IResult;

pub fn parse_keyword(input: &[u8]) -> IResult<&[u8], Lexeme> {
    alt((
        parse_break,
        parse_do,
        parse_else,
        parse_elseif,
        parse_end,
        parse_false,
        parse_for,
        parse_function,
        parse_goto,
        parse_if,
        parse_in,
        parse_local,
        parse_nil,
        parse_not,
        parse_or,
        parse_repeat,
        parse_return,
        parse_then,
        parse_true,
        parse_until,
        parse_while,
    ))(input)
}

pub fn parse_symbol(input: &[u8]) -> IResult<&[u8], Lexeme> {
    // these are sorted by longest to smallest to be sure to not mix things up
    // between "<" and "<<" for example
    // we need to split this in multiple parts because rust does not
    // allow tuple this long
    alt((
        // 3 chars
        alt((parse_logical_and, parse_logical_or, parse_logical_not)),
        // 2 chars
        (alt((
            parse_equality,
            parse_inequality,
            parse_concat,
            parse_idiv,
            parse_right_shift,
            parse_left_shift,
            parse_less_or_equal,
            parse_more_or_equal,
        ))),
        (alt((
            parse_plus,
            parse_logical_not,
            parse_semi_colon,
            parse_minus,
            parse_mul,
            parse_div,
            parse_mod,
            parse_length,
            parse_exponentiation,
            parse_unary_not,
            parse_binary_and,
            parse_binary_or,
            parse_open_parenthesis,
            parse_close_parenthesis,
            parse_less_than,
            parse_more_than,
            parse_assign,
        ))),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_keyword() {
        assert_eq!(parse_keyword(b"do "), Ok((&b" "[..], Lexeme::Do)));
        assert_eq!(parse_keyword(b"end "), Ok((&b" "[..], Lexeme::End)));
        assert_eq!(parse_keyword(b"goto "), Ok((&b" "[..], Lexeme::Goto)));
        assert_eq!(parse_keyword(b"in "), Ok((&b" "[..], Lexeme::In)));
        assert_eq!(parse_keyword(b"repeat "), Ok((&b" "[..], Lexeme::Repeat)));
        assert_eq!(parse_keyword(b"return "), Ok((&b" "[..], Lexeme::Return)));
    }

    #[test]
    fn test_parse_symbol() {
        assert_eq!(parse_symbol(b"+ "), Ok((&b" "[..], Lexeme::Plus)));
        assert_eq!(parse_symbol(b">> "), Ok((&b" "[..], Lexeme::RightShift)));
        assert_eq!(parse_symbol(b"> "), Ok((&b" "[..], Lexeme::MoreThan)));
        assert_eq!(parse_symbol(b">>>"), Ok((&b">"[..], Lexeme::RightShift)));
    }
}
