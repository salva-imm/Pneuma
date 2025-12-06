use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub(crate) enum TokenType {
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
pub(crate) enum LiteralObject {
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
pub(crate) struct Token {
    ttype: TokenType,
    lexeme: String,
    literal: Option<LiteralObject>,
    line: usize
}

impl Token {
    pub(crate) fn new(
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
    pub(crate) fn eof(line: usize) -> Self {
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