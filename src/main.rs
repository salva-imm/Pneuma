use std::{fmt, io};
use std::io::{BufReader, BufRead};
use std::fs::{read, read_to_string, File};
use std::env::args;
use std::fmt::{write, Formatter};
use std::process::exit;


#[derive(Debug)]
enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String,
    Number,
    // keywords
    And,
    Class,
    Else,
    False,
    Pn,
    For,
    If,
    None,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Let,
    While,
    EOF
}

#[derive(Debug)]
enum LiteralObject {
    Num(f64),
    Str(String),
    // Char(char),
    None,
    Bool(bool)
}

impl fmt::Display for LiteralObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LiteralObject::Num(v) => write!(f, "{v}"),
            LiteralObject::Str(v) => write!(f, "\"{v}\""),
            LiteralObject::None => write!(f, "None"),
            LiteralObject::Bool(v) =>  write!(f, "{v}"),
        }
        // write!(f, )
    }
}

#[derive(Debug)]
struct Token {
    ttype: TokenType,
    lexeme: String,
    literal: Option<LiteralObject>,
    line: usize
}

impl Token {
    fn new(&self,
           ttype: TokenType,
           lexeme: String,
           literal: Option<LiteralObject>,
           line: usize) -> Self {
        Token {
            ttype,
            lexeme,
            literal,
            line
        }
    }
    fn eof(line: usize) -> Self {
        Token {
            ttype: TokenType::EOF,
            lexeme: "".to_string(),
            literal: None,
            line
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,
               "{:#?} {:#?} {:#?} {:#?}",
               self.ttype,
               self.lexeme,
               self.literal,
               self.line
        )
    }
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize
}


impl Scanner {
    fn new(
        &self,
        source: String,
    ) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1
        }
    }
    fn is_end(&self) -> bool {
        self.current >= self.source.len()
    }
    fn scan_tokens(&self) {
        todo!()
    }
    fn generate_tokens(&mut self) -> Result<bool, PneumaError> {
        while self.is_end() == false {
            self.start = self.current;
            let _ =self.scan_tokens();
        }
        self.tokens.push(Token::eof(self.line));
        Ok(true)
    }
}

#[derive(Debug)]
struct PneumaError {
    line: usize,
    message: String
}

impl PneumaError {
    fn new(&self, line: usize, message: String) -> Self {
        PneumaError {
            line,
            message
        }
    }
    fn report(&self) {
        eprintln!("~~~ line {} ~~~ \n Error {}", self.line, self.message);
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() > 2 {
        println!("Usage: ")
    }else if args.len() == 2 {
        println!("running ...");
        let _ = run_file(String::from(args[1].to_string()));
    }else {
        run_shell()
    }
}

fn run_file(filename: String) -> io::Result<()> {
    let buf = read_to_string(filename)?;
    match run(buf.as_str()) {
        Ok(_) => {},
        Err(e) => {
            e.report();
            exit(65)
        }
    }
    Ok(())
}

fn run_shell() {
    let stdin = io::stdin();
    println!("pn>");
    for line in stdin.lock().lines() {
        if let Ok(li) = line {
            if li.is_empty() {
                break;
            }
            match run(li.as_str()) {
                Ok(_) => {},
                Err(e)=> {
                    e.report()
                }
            }
        } else {
            break;
        }

    }
}

fn run(source: &str) -> Result<String, PneumaError>{
    Ok("fine".to_string())

}
