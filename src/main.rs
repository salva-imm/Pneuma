use std::{char, fmt, io};
use std::io::{BufReader, BufRead, stdout, Write};
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
    fn new(
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

#[derive(Debug)]
struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize
}


impl Scanner {
    fn new(
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
    fn advance(&mut self) -> Result<char, PneumaError> {
        if let Some(c) = self.source.chars().nth(self.current) {
            self.current += 1;
            Ok(c)
        }else{
            Err(
                PneumaError {
                    line: self.current,
                    message: "Out of index".to_string(),
                }
            )
        }

    }

    fn add_token(&mut self, ttype: TokenType, literal: Option<LiteralObject>) -> Result<(), PneumaError> {
        let lexeme: String = self.source[self.start..self.current].to_string();
        self.tokens.push(
            Token::new(ttype, lexeme, literal, self.line)
        );
        Ok(())
    }
    fn peek(&self) -> Option<char> {
        if self.is_end() {
            return Some('\0')
        };
        self.source.chars().nth(self.current)
    }
    fn scan_tokens(&mut self) -> Result<(), PneumaError>{
        let c = self.advance()?;
        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => {
                if self.is_second_match('=')? {
                    self.add_token(TokenType::BangEqual, None)
                } else {
                    self.add_token(TokenType::Bang, None)
                }
            },
            '=' => {
                if self.is_second_match('=')? {
                    self.add_token(TokenType::EqualEqual, None)
                } else {
                    self.add_token(TokenType::Equal, None)
                }
            },
            '<' => {
                if self.is_second_match('=')? {
                    self.add_token(TokenType::LessEqual, None)
                } else {
                    self.add_token(TokenType::Less, None)
                }
            },
            '>' => {
                if self.is_second_match('=')? {
                    self.add_token(TokenType::GreaterEqual, None)
                } else {
                    self.add_token(TokenType::Greater, None)
                }
            },
            '/' => {
                if self.is_second_match('/')? {
                    while self.peek() != Option::from('\n') && !self.is_end(){
                        self.advance()?;
                    }
                    Ok(())
                } else {
                    self.add_token(TokenType::Slash, None)
                }
            },
            ' ' | '\t' | '\r' => {Ok(())},
            '\n' => {
              self.line += 1;
                Ok(())
            },
            '"' => {
                self.string()
            }
            _ => Err(
                PneumaError {
                    line: self.line,
                    message: "Unexpected Character".to_string()
                }
            )
        }
    }
    fn string(&self) -> Result<(),PneumaError>{
        todo!()
    }
    fn generate_tokens(&mut self) -> Result<bool, PneumaError> {
        while self.is_end() == false {
            self.start = self.current;
            match self.scan_tokens() {
                Ok(_) => {}
                Err(e) => {
                    e.report();
                }
            }
        }
        self.tokens.push(Token::eof(self.line));
        Ok(true)
    }
    fn is_second_match(&mut self, expected: char) -> Result<bool, PneumaError>{
        if self.is_end(){
            return Ok(false)
        }
        if let Some(current_c) = self.source.chars().nth(self.current){
            if current_c != expected{
                return Ok(false)
            }
        }
        self.current += 1;
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
    match run(buf) {
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
    print!("pn> ");
    let _ = stdout().flush().expect("Failed to flush stdout");
    for line in stdin.lock().lines() {
        if let Ok(li) = line {
            // if li.is_empty() {
            //     break;
            // }
            match run(li) {
                Ok(_) => {},
                Err(e)=> {
                    e.report()
                }
            }
        } else {
            break;
        }
        print!("pn> ");
        let _ = stdout().flush().expect("Failed to flush stdout");

    }
}

fn run(source: String) -> Result<(), PneumaError>{
    let mut scanner = Scanner::new(source);
    let _status = scanner.generate_tokens()?;
    println!("{:#?}", &scanner);
    // for t in &scanner.tokens {
    //     println!("{:#?}", t);
    // }
    Ok(())
}
