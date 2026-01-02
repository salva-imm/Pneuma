use super::utils::PneumaError;
use super::token::{
    Token, TokenType, LiteralObject
};

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize
}


impl Scanner {
    pub fn new(
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

    fn add_token(
        &mut self,
        ttype: TokenType,
        literal: Option<LiteralObject>,
    ) -> Result<(), PneumaError> {
        let lexeme = self.source[self.start..self.current].to_string();

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
    fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.source.len() {
            return Some('\0')
        }
        self.source.chars().nth(self.current+1)
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
            },
            '0'..='9' => {
                self.number()
            },
            _ => Err(
                PneumaError {
                    line: self.line,
                    message: "Unexpected Character".to_string()
                }
            )
        }
    }
    fn string(&mut self) -> Result<(),PneumaError> {
        while self.peek() != Option::from('"') && !self.is_end() {
            if self.peek() == Option::from('\n') {
                self.line += 1
            }
            let _ = self.advance();
        }
        if self.is_end() {
            return Err(PneumaError {
                message: "Unterminated string.".to_string(),
                line: self.line
            })
        }
        let _ = self.advance();
        let string_value = self.source[self.start+1..self.current-1].to_string();
        let _ =self.add_token(TokenType::String, Some(LiteralObject::Str(string_value)));
        Ok(())
    }
    pub(crate) fn generate_tokens(&mut self) -> Result<bool, PneumaError> {
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
    fn number(&mut self) -> Result<(), PneumaError> {
        while self.is_digit(self.peek().unwrap()) {
            let _ = self.advance();
        }
        if self.peek() == Option::from('.') && self.is_digit(self.peek_next().unwrap()){
            let _ = self.advance();
            while self.is_digit(self.peek().unwrap()) {
                let _ = self.advance();
            }
        }
        let number_value = self.source[self.start..self.current].to_string();
        let num = number_value.parse::<f64>().unwrap();
        let _ =self.add_token(TokenType::Number, Some(LiteralObject::Num(num)));
        Ok(())
    }
    fn is_digit(&self, c: char) -> bool {
        if c >= '0' && c <= '9' {
            return true;
        }
        false
    }
    fn is_second_match(&mut self, expected: char) -> Result<bool, PneumaError> {
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