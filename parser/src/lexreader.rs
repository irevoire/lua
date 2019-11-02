use lexer::Lexeme;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufReader;

pub struct LexReader {
    lexer: lexer::Lexer,
    unread: Vec<Lexeme>,
}

impl LexReader {
    pub fn new(lexer: lexer::Lexer) -> Self {
        LexReader {
            lexer,
            unread: Vec::new(),
        }
    }

    pub fn from(path: &std::path::Path) -> io::Result<Self> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        let lexer = lexer::Lexer::new(reader)?;

        Ok(Self::new(lexer))
    }

    pub fn read(&mut self) -> Result<Lexeme, Box<dyn Error>> {
        if let Some(lex) = self.unread.pop() {
            Ok(lex)
        } else if let Some(lex) = self.lexer.next() {
            lex
        } else {
            Err(EoFError.into())
        }
    }

    pub fn unread(&mut self, lex: Lexeme) {
        self.unread.push(lex);
    }
}

/// create a custom error to catch EOF
#[derive(Debug)]
pub struct EoFError;

impl std::fmt::Display for EoFError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EOF was reached, no more lexeme")
    }
}

impl Error for EoFError {
    fn description(&self) -> &str {
        "EOF was reached"
    }
}
