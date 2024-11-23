use std::fmt::{self, Display, Formatter};

#[derive(Clone)]
pub enum KeywordType {
    Print,
}

#[derive(Clone)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
}

#[derive(Clone)]
pub enum TokenType {
    Number,
    String,
    Boolean,
    Operator(Op),
    Keyword(KeywordType),
    Identifiers,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            TokenType::Keyword(KeywordType::Print) => write!(f, "Keyword::Print"),
            TokenType::Operator(Op::Add) => write!(f, " is <Operator/Add>"),
            TokenType::Number => write!(f, " is Number"),
            _ => write!(f, "Unidentified <TokenType>"),
        }
    }
}

impl Copy for TokenType {}

impl Copy for KeywordType {}
impl Copy for Op {}
#[derive(Clone)]
pub struct Token<'a> {
    pub lexeme: &'a str,
    pub literal_type: TokenType,
}
impl<'a> Copy for Token<'a> {}
impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[Token] {{ Lexeme: {}, literal_type {} }}", self.lexeme, self.literal_type)
    }
}
impl<'a> Token<'a> {
    pub fn is_numeric(&self) -> bool {
        self.lexeme.chars().all(|c| c.is_numeric())
    }
}