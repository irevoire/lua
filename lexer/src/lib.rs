mod lexeme;

pub use lexeme::*;
use nom::branch::alt;
use nom::character::streaming::multispace0;
use nom::{error::ErrorKind, Err, IResult};
use std::io::{BufReader, Read};

const BUF_SIZE: usize = 32;

pub struct Lexer {
    reader: BufReader<std::fs::File>,
    buf: [u8; BUF_SIZE], // <- the lifetime should be here
    idx: usize,
}

impl Lexer {
    pub fn new(mut reader: BufReader<std::fs::File>) -> Self {
        let mut buf = [0; BUF_SIZE];
        reader.read(&mut buf);
        unsafe {
            Lexer {
                reader,
                buf,
                idx: 0,
            }
        }
    }
}

pub fn lex(input: &[u8]) -> IResult<&[u8], Lexeme> {
    // println!("called with: buf=\"{:?}\"", std::str::from_utf8(input));
    // first remove the useless space that could exist at the start
    let (rem, _) = multispace0(input)?;
    // these are sorted by longest to smallest to be sure to not mix things up
    // between "<" and "<<" for example
    // we need to split this in multiple parts because rust does not
    // allow tuple this long
    alt((
        // keywords
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
        )),
        // 2 chars
        (alt((
            parse_comment,
            parse_equality,
            parse_inequality,
            parse_concat,
            parse_idiv,
            parse_right_shift,
            parse_left_shift,
            parse_less_or_equal,
            parse_more_or_equal,
        ))),
        // 1 char
        (alt((
            parse_plus,
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
        // user input
        (alt((parse_int, parse_float, parse_string, parse_identifier))),
    ))(rem)
}

impl Iterator for Lexer {
    type Item = Result<Lexeme, Box<dyn std::error::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        // println!("current buf is: {:?}", self.idx);
        let slice = &self.buf[self.idx..];
        let initial_size = slice.len();

        match lex(slice) {
            Err(Err::Incomplete(_)) => {
                // 1. move the part we were unable to parse to the start of the buffer
                let mut old_idx = self.idx;
                self.idx = 0;
                for i in old_idx..self.buf.len() {
                    self.buf[self.idx] = self.buf[i];
                    self.idx += 1;
                }
                // println!("buffer after move is: {:?}", std::str::from_utf8(&self.buf));
                let read = self.reader.read(&mut self.buf[self.idx..]);
                match read {
                    Ok(0) => return None, // EOF
                    _ => (),
                };
                // println!("buffer after read is: {:?}", std::str::from_utf8(&self.buf));
                // println!("read called with idx: {}", self.idx);
                self.idx = 0; // put back the pointer to where it could not parse
                self.next()
            }
            Err(e) => {
                // println!("{:?}", e);
                Some(Err("Error".into()))
            }
            Ok((rem, lex)) => {
                // we compute the difference between the two slice to know how
                // much we avanced
                self.idx += initial_size - rem.len();
                Some(Ok(lex))
            }
            _ => None,
        }
    }
}
