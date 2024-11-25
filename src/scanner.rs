use crate::token::{self, Token, TokenType};

pub struct Scanner<'a> {
    pub source: &'a str,
    pub start: usize,
    pub current: usize,
    pub line: u32,
}

#[allow(unused)]
impl<'a> Scanner<'a> {
    pub fn init_scanner(source: &'a str) -> Scanner<'a> {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn is_at_end(&mut self) -> bool {
        match self.current_char() {
            Some(c) if c == '\0' => true,
            None => true,
            _ => false,
        }
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            start: &self.source[self.start..self.current],
            length: self.current - self.start,
            line: self.line,
        }
    }

    fn error_token(&self, message: &'a str) -> Token {
        Token {
            token_type: TokenType::Error((self.start, self.current)),
            start: message,
            length: message.len(),
            line: self.line,
        }
    }
    pub fn get_lexeme(&self, loc: (usize, usize)) -> &'a str{
        return &self.source[loc.0..loc.1]
    }
    fn current_char(&mut self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            match c {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                _ => break,
            }
        }
    }

    fn string(&mut self) -> Token {
        while let Some(c) = self.current_char() {
            if self.is_at_end() || c == '"' {
                break;
            }
            if c == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            // let err_msg  = format!("Unterminated string: `{}`", &self.source[self.start..self.current]);
            return self.error_token("Unterminated String");
        }
        self.advance();
        self.make_token(TokenType::String)
    }
    fn number(&mut self) -> Token{
        while let Some(c) = self.current_char() {
            
            match c.is_numeric() {
                true => {
                    self.advance();
                },
                false => break
            }
        }
        return self.make_token(TokenType::Number)
    }
    fn check_keyword(&self, rest: &str, token_type: TokenType) -> TokenType {
        let lexeme = self.get_lexeme((self.start + 1, self.current));
        //println!("check_keyword: {} {}", lexeme, rest);
        if lexeme == rest {
            return token_type
        }
        return TokenType::Identifier
    }
    fn identifier_type(&self) -> TokenType {
        let first = self.source.chars().nth(self.start);
        let lexeme_length = self.current - self.start;
        //println!("first: {}", first.unwrap());
        match first {
            Some(c) => {
                match c {
                    'a' => return self.check_keyword("nd", TokenType::And),
                    'c' => return self.check_keyword("lass", TokenType::Class),
                    'e' => return self.check_keyword("lse", TokenType::Else),
                    'f' => {
                        if lexeme_length > 1 {
                            match self.source.chars().nth(self.start + 1).unwrap() {
                                'a' => return self.check_keyword("alse", TokenType::False),
                                'o' => return self.check_keyword("or", TokenType::For),
                                'n' => return self.check_keyword("n", TokenType::Fun),
                                _ => {
                                    
                                    return TokenType::Identifier 
                                }
                            }
                        }
                    }
                    'i' => return self.check_keyword("f", TokenType::If),
                    'n' => return self.check_keyword("il", TokenType::Nil),
                    'o' => return self.check_keyword("r", TokenType::Or),
                    'p' => return self.check_keyword("rint", TokenType::Print),
                    'r' => return self.check_keyword("eturn", TokenType::Return),
                    's' => return self.check_keyword("uper", TokenType::Super),
                    't' => {
                        if lexeme_length > 1 {
                            match self.source.chars().nth(self.start + 1).unwrap() {
                                'h' => return self.check_keyword("his", TokenType::This),
                                'r' => return self.check_keyword("rue", TokenType::True),
                                _ => {
                                    
                                    return TokenType::Identifier 
                                }
                            }
                        }
                    }
                    'v' => return self.check_keyword("ar", TokenType::Var),
                    'w' => return self.check_keyword("hile", TokenType::While),
                    _ => return TokenType::Identifier
                }
            },
            _ => return TokenType::Error((self.start, self.current))
        }
        return TokenType::Identifier
    }
    fn identifier(&mut self) -> Token {
        while let Some(c) = self.current_char() {
            //println!("{}", c);
            if c.is_alphanumeric() { 
                self.advance(); 
            } else { break }
        }
        return self.make_token(self.identifier_type());
    }

    pub fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current - 1)
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.current_char().unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();

        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(TokenType::EOF);
        }

        let advancer = self.advance();
        
        match advancer {
            Some(c) => match c {
                c if c.is_alphabetic() => self.identifier(),
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => self.number(),
                '(' => self.make_token(TokenType::LeftParen),
                ')' => self.make_token(TokenType::RightParen),
                '{' => self.make_token(TokenType::LeftBrace),
                '}' => self.make_token(TokenType::RightBrace),
                ';' => self.make_token(TokenType::Semicolon),
                ',' => self.make_token(TokenType::Comma),
                '.' => self.make_token(TokenType::Dot),
                '-' => self.make_token(TokenType::Minus),
                '+' => self.make_token(TokenType::Plus),
                '/' => self.make_token(TokenType::Slash),
                '*' => self.make_token(TokenType::Star),
                '!' => {
                    if self.match_next('=') {
                        self.make_token(TokenType::BangEqual)
                    } else {
                        self.make_token(TokenType::Bang)
                    }
                }
                '=' => {
                    if self.match_next('=') {
                        self.make_token(TokenType::EqualEqual)
                    } else {
                        self.make_token(TokenType::Equal)
                    }
                }
                '<' => {
                    if self.match_next('=') {
                        self.make_token(TokenType::LessEqual)
                    } else {
                        self.make_token(TokenType::Less)
                    }
                }
                '>' => {
                    if self.match_next('=') {
                        self.make_token(TokenType::GreaterEqual)
                    } else {
                        self.make_token(TokenType::Greater)
                    }
                }
                '"' => self.string(),
                _ => self.error_token("Unexpected character."),
            },
            None => self.error_token("Unexpected character."),
        }
    }
}
