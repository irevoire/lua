use grammar::Token;
use logos::Logos;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut args = env::args();
    if args.len() != 2 {
        println!("usage: cargo run [filename]");
        return Ok(());
    }
    args.next(); // executable name
    let mut f = File::open(args.next().unwrap())?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    let buffer = std::str::from_utf8(&buffer).unwrap();

    let mut lexer = Token::lexer(buffer);
    loop {
        let lex = &lexer.token;

        match lex {
            Token::ParseEnd => break,
            Token::ParseError => {
                println!("\n\x1B[31mError on:\x1B[m\"{}\"", lexer.slice());
                return Ok(());
            }
            Token::NewLine => println!(""),
            Token::String | Token::Var | Token::Comments | Token::Int => {
                print!("\x1B[32m{:?}\x1B[m({}) ", lex, lexer.slice())
            }
            t => print!("\x1B[32m{:?}\x1B[m ", t),
        }
        lexer.advance();
    }
    println!("\n================================");
    println!("\nFile successfully tokenized");
    Ok(())
}
