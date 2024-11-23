// mod token;      // Importing the token module
// mod parser;     // Importing the parser module
// mod error;      // Importing the error module

use std::fmt;
// use parser::parse_binary;


struct Token<'a>{
    token_type: TokenType,
    start: &'a str,
    length: usize,
    line: u32
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
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

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Type,
    IntTag,
    FloatTag,
    StringTag,
    Struct,

    Error,
    DotDot,
    Len,
    PlusPlus,
    Percent,
    EOF,
}

impl TokenType {
    pub fn to_u32(self) -> u32 {
        // Use the `as` keyword to cast the enum variant to its underlying value.
        self as u32
    }
}
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Match each variant and return the variant name as a string
        match self {
            TokenType::LeftParen => write!(f, "LeftParen"),
            TokenType::RightParen => write!(f, "RightParen"),
            TokenType::LeftBrace => write!(f, "LeftBrace"),
            TokenType::RightBrace => write!(f, "RightBrace"),
            TokenType::LeftBracket => write!(f, "LeftBracket"),
            TokenType::RightBracket => write!(f, "RightBracket"),
            TokenType::Comma => write!(f, "Comma"),
            TokenType::Dot => write!(f, "Dot"),
            TokenType::Minus => write!(f, "Minus"),
            TokenType::Plus => write!(f, "Plus"),
            TokenType::Semicolon => write!(f, "Semicolon"),
            TokenType::Slash => write!(f, "Slash"),
            TokenType::Star => write!(f, "Star"),
            TokenType::Error => write!(f, "Error"),

            TokenType::Bang => write!(f, "Bang"),
            TokenType::BangEqual => write!(f, "BangEqual"),
            TokenType::Equal => write!(f, "Equal"),
            TokenType::EqualEqual => write!(f, "EqualEqual"),
            TokenType::Greater => write!(f, "Greater"),
            TokenType::GreaterEqual => write!(f, "GreaterEqual"),
            TokenType::Less => write!(f, "Less"),
            TokenType::LessEqual => write!(f, "LessEqual"),

            TokenType::Identifier => write!(f, "Identifier"),
            TokenType::String => write!(f, "String"),
            TokenType::Number => write!(f, "Number"),

            TokenType::And => write!(f, "And"),
            TokenType::Class => write!(f, "Class"),
            TokenType::Else => write!(f, "Else"),
            TokenType::False => write!(f, "False"),
            TokenType::For => write!(f, "For"),
            TokenType::Fun => write!(f, "Fun"),
            TokenType::If => write!(f, "If"),
            TokenType::Nil => write!(f, "Nil"),
            TokenType::Or => write!(f, "Or"),
            TokenType::Print => write!(f, "Print"),
            TokenType::Return => write!(f, "Return"),
            TokenType::Super => write!(f, "Super"),
            TokenType::This => write!(f, "This"),
            TokenType::True => write!(f, "True"),
            TokenType::Var => write!(f, "Var"),
            TokenType::While => write!(f, "While"),

            TokenType::Type => write!(f, "Type"),
            TokenType::IntTag => write!(f, "IntTag"),
            TokenType::FloatTag => write!(f, "FloatTag"),
            TokenType::StringTag => write!(f, "StringTag"),
            TokenType::Struct => write!(f, "Struct"),

            TokenType::Error => write!(f, "Error"),
            TokenType::DotDot => write!(f, "DotDot"),
            TokenType::Len => write!(f, "Len"),
            TokenType::PlusPlus => write!(f, "PlusPlus"),
            TokenType::Percent => write!(f, "Percent"),
            TokenType::EOF => write!(f, "EOF"),
        }
    }
}
struct Scanner {
    source: &'static str,
    start: usize , // slice of current source code - points to start of current lexeme
    current: usize, // points to current character being scanned
    line: u32
}
fn init_scanner(source: &'static str) -> Scanner{
    Scanner {
        source: &source,
        start: 0,
        current: 0,
        line: 1
    }
}
fn offset_from(slice: &[u8], ptr: *const u8) -> usize {
    // Calculate the number of bytes between the slice start and the pointer
    ((ptr as usize) - (slice.as_ptr() as usize)) / std::mem::size_of::<u8>()
}
impl <'a>Scanner {
    fn is_at_end(&mut self) -> bool {

        match self.current_char() {
            Some(c) if c == '\0' => true,
            None => true,
            _ => false
        }
    }
    fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type: token_type,
            start: &self.source[self.start..self.current],
            length: self.current - self.start as usize,
            line: self.line
        }
    }
    fn error_token(&self, message: &'static str) -> Token{
        Token {
            token_type: TokenType::Error,
            start: message,
            length: message.len(),
            line: self.line
        }
    } 
    fn current_char(&mut self) -> Option<char> {
        return self.source.chars().nth(self.current)
    }
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {

            match c {
                ' ' | '\t' | '\r' => {
                    self.advance();
                },
                '\n' => {
                    self.line+=1;
                    self.advance();
                }
                _ => break
            }
        }
    }
    fn string(&mut self) -> Token {
        while let Some(c) = self.current_char() {
            if self.is_at_end() || c == '"' { break }
            if c == '\n' {self.line += 1}
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string")
        }
        self.advance();
        return self.make_token(TokenType::String);
    }
    fn advance(&mut self) -> Option<char>{
        self.current += 1;
        return self.source.chars().nth(self.current - 1);
    }
    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false };
        if self.current_char().unwrap() != expected { return false }
        self.current += 1;
        true

    }
    fn scan_token(&mut self) -> Token{
        self.skip_whitespace();

        self.start = self.current;
        if self.is_at_end() {
            self.make_token(TokenType::EOF);
        }
        let advancer = self.advance();
        match advancer  {
            Some(c) => {

                match c {
                    '(' => self.make_token(TokenType::LeftParen),
                    ')' => self.make_token(TokenType::RightParen),
                    '{' => self.make_token(TokenType::LeftBracket),
                    '}' => self.make_token(TokenType::RightBracket),
                    ';' => self.make_token(TokenType::Semicolon),
                    ',' => self.make_token(TokenType::Comma),
                    '.' => self.make_token(TokenType::Dot),
                    '-' => self.make_token(TokenType::Minus),
                    '+' => self.make_token(TokenType::Plus),
                    '/' => self.make_token(TokenType::Slash),
                    '*' => self.make_token(TokenType::Star),
                    '!' => {
                        match self.match_next('=') {
                            true    => return self.make_token(TokenType::BangEqual),
                            false   => return self.make_token(TokenType::Bang)
                        }
                    },
                    '=' => {
                        match self.match_next('=') {
                            true    => return self.make_token(TokenType::EqualEqual),
                            false   => return self.make_token(TokenType::Equal)
                        }
                    },
                    '<' => {
                        match self.match_next('=') {
                            true    => return self.make_token(TokenType::LessEqual),
                            false   => return self.make_token(TokenType::Less)
                        }
                    },
                    '>' => {
                        match self.match_next('=') {
                            true    => return self.make_token(TokenType::GreaterEqual),
                            false   => return self.make_token(TokenType::Greater)
                        }
                    },
                    '"' => self.string(),

                    _ => {return self.error_token("Unexpected character.")}
                }
            }
            None => {return self.error_token("Unexpected character.")}
        }
        

    }
    
}
fn main() {

    let scanner = &mut init_scanner("++-=,.!\"aaa\0");

    while let s = scanner.scan_token() {
        println!("{} {}", s.token_type, s.start);
        match s.token_type {
            TokenType::EOF => { break }
            _ => {}
        }
        if scanner.is_at_end() { break }
        // Process the token `t` here
    }
    println!("{}", scanner.is_at_end());

}
