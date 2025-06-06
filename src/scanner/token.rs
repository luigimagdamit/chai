use std::fmt;
// use parser::parse_binary;


#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'a>{
    pub token_type: TokenType,
    pub start: &'a str,
    pub length: usize,
    pub line: u32
}
impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "token_type: {} start: {} length: {} line: {}", 
               self.token_type, self.start, self.length, self.line)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(unused)]
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
    Colon,
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

    Error((usize, usize)),
    DotDot,
    Len,
    PlusPlus,
    Percent,
    EOF,
}
#[allow(unused)]
impl TokenType {
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
            TokenType::Colon => write!(f, "Colon"),
            TokenType::Semicolon => write!(f, "Semicolon"),
            TokenType::Slash => write!(f, "Slash"),
            TokenType::Star => write!(f, "Star"),

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

            TokenType::Error(_) => write!(f, "Error"),
            TokenType::DotDot => write!(f, "DotDot"),
            TokenType::Len => write!(f, "Len"),
            TokenType::PlusPlus => write!(f, "PlusPlus"),
            TokenType::Percent => write!(f, "Percent"),
            TokenType::EOF => write!(f, "EOF"),
        }
    }
}
