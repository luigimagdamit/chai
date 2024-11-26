mod token;    // Import the Token and TokenType module
mod scanner;  // Import the Scanner module
use scanner::Scanner;
use token::{TokenType, ErrorCode, Token};

enum Precedence {
    PrecNone,
    PrecAssignment,
    PrecAnd,
    PrecEquality,
    PrecComparison,
    PrecTerm,
    PrecFactor,
    PrecUnary,
    PrecCall,
    PrecPrimary
}
impl Precedence {
    // Convert a number to a Precedence enum variant
    fn from_u32(value: u32) -> Precedence {
        match value {
            0 => (Precedence::PrecNone),
            1 => (Precedence::PrecAssignment),
            2 => (Precedence::PrecAnd),
            3 => (Precedence::PrecEquality),
            4 => (Precedence::PrecComparison),
            5 => (Precedence::PrecTerm),
            6 => (Precedence::PrecFactor),
            7 => (Precedence::PrecUnary),
            8 => (Precedence::PrecCall),
            9 => (Precedence::PrecPrimary),
            _ => Precedence::PrecNone

        }
    }
    fn to_u32(&self) -> u32 {
        match self {
            Precedence::PrecNone => 0,
            Precedence::PrecAssignment => 1,
            Precedence::PrecAnd => 2,
            Precedence::PrecEquality => 3,
            Precedence::PrecComparison => 4,
            Precedence::PrecTerm => 5,
            Precedence::PrecFactor => 6,
            Precedence::PrecUnary => 7,
            Precedence::PrecCall => 8,
            Precedence::PrecPrimary => 9,
        }
    }
}
struct Parser<'a>{
    current: Option<token::Token<'a>>,
    previous: Option<token::Token<'a>>,
    scanner: Scanner<'a>,
    had_error: bool,
    panic_mode: bool,
    left_hand: Option<Leaf>,
    right_hand: Option<Leaf>
}
impl<'a>Parser <'a>{
    fn error_at(&mut self, token: &Token, message: &str) {
        self.panic_mode = true;
        let stderr = format!("Line: {} - ", token.line);
        match token.token_type {
            TokenType::EOF => {
                println!("{} at end of file", token.start)
            },
            TokenType::Error(loc) => {
                println!("{} {} at `{}...`", stderr, token.start, &self.scanner.get_lexeme(loc));
            },
            _ => {
                println!("{} {}  at `{}`", stderr, message, token.start);
            }
        }
        if self.panic_mode {
            return
        }
    } 
    fn advance(&mut self) {
        self.previous = self.current.take();
        
        // println!("Scanner State");
        loop {
            let token = self.get_token();
            match token.token_type {
                TokenType::Error(_)=> {
                    self.error_at(&token, token.start);
                },
                _ => {
                    self.current = Some(token);
                    // match self.previous {
                    //     Some(t) => println!("\t<{}>", t),
                    //     _ => println!("\t<no token>")
                    // }
                    // match self.current {
                    //     Some(t) => println!("\t<{}>", t),
                    //     _ => println!("<\tno token>")
                    // }

                    break;
                }
            }
        }

    }
    fn consume(&mut self, token_type: TokenType, message: &str) {
        if let Some(token) = self.current {
            match token.token_type {
                _ if token.token_type != token_type => {
                    self.error_at(&token, message);
                },
                _ => {
                    self.advance();
                }
            }
        }
    }
    fn get_token(&mut self) -> Token<'a> {
        return self.scanner.scan_token().clone()
    }
    fn init_parser(source: &'static str) -> Parser<'_> {
        Parser {
            current: None,
            previous: None,
            scanner: Scanner::init_scanner(&source),
            panic_mode: false,
            had_error: false,
            left_hand: None,
            right_hand: None
        }
    }
    fn compile(&mut self) {
        self.advance();
        expression(self);
        self.consume(TokenType::EOF, "Expect end of expression");

    }
}

enum DataType {
    Integer(String)
}
enum Operator {
    Add
}
struct Leaf {
    left: String,
    right: String,
    data_type: DataType
}
impl Leaf {
    fn print_leaf(&self) {
        println!("<leaf> <left: {}> <right: {}>", self.left, self.right);
    }
}
struct Root {
    left: Leaf,
    right: Leaf,
    operator: Operator
}
fn parse_number(parser: &mut Parser) {
    let value = parser.previous.unwrap().start;
    let number_leaf = Leaf {
        left: String::from(format!("i32 {}", value)),
        right: String::from(value),
        data_type: DataType::Integer((String::from(value)))
    };
    &number_leaf.print_leaf();
    match parser.left_hand {
        None => parser.left_hand = Some(number_leaf),
        Some(_) => parser.right_hand = Some(number_leaf)
    }
    
    println!("<number: {}>", value)
}
fn expression(parser: &mut Parser) {
    parse_precedence(parser, Precedence::PrecAssignment);
}
fn parse_precedence(parser: &mut Parser, precedence: Precedence) {
    parser.advance();
    // println!("AHHH {}", parser.previous.unwrap());
    if let Some(prev) = parser.previous {
        let prefix_rule = get_rule(prev.token_type).prefix;
        match prefix_rule {
            None => { 
                 panic!()
            },
            Some(prefix_fn) => {
                prefix_fn(parser);
            }
        }
        if let Some(curr) = parser.current {
            // println!("aaaaa{}", curr.token_type);
            // println!("Status {} {}", parser.previous.unwrap(), parser.current.unwrap());
            // println!("Prec {} {}", precedence.to_u32(), get_rule(curr.token_type).precedence.to_u32());
            while precedence.to_u32() <= get_rule(curr.token_type).precedence.to_u32() {
                
                parser.advance();

                if let Some(infix_rule) = get_rule(parser.previous.unwrap().token_type).infix {

                    infix_rule(parser);
                } else {
                    break
                }
            }
        }
    }
    
}
fn parse_binary(parser: &mut Parser) {
    if let Some(token) = parser.previous {
       
        let operator_type = token.token_type;
        let rule_fn = get_rule(operator_type);
        let new_prec = rule_fn.precedence as u32;
        parse_precedence(parser, Precedence::from_u32(new_prec + 1));

        if let Some(left) = &parser.left_hand {
            left.print_leaf();
        }
        if let Some(right) = &parser.right_hand {
            right.print_leaf();
        }
        match operator_type {
            TokenType::Plus => println!("<plus>"),
            _ => {}
        }


    }
    
}
fn parse_grouping(parser: &mut Parser) {
    expression(parser);
    parser.consume(TokenType::RightParen, "Expect ')' after expression");
}


fn get_rule<'a>(token_type: TokenType) -> ParseRule<'a> {
    // println!("GetRule: {}", token_type);
    match token_type {
        TokenType::Plus => {

            let add_rule = ParseRule { prefix: None, infix: Some(parse_binary), precedence: Precedence::PrecTerm };
            if let Some(r) = add_rule.infix {

            }
            return add_rule
        },
        TokenType::Number => ParseRule { prefix: Some(parse_number), infix: None, precedence: Precedence::PrecNone },
        TokenType::EOF => ParseRule {prefix: None, infix: None, precedence: Precedence::PrecNone },
        _ => ParseRule { prefix: None, infix: None, precedence: Precedence::PrecNone } 
    }
}
struct ParseRule<'a>{
    prefix: Option<ParseFn<'a>>,
    infix: Option<ParseFn<'a>>,
    precedence: Precedence,
}
type ParseFn<'a> = fn(&'a mut Parser);
fn main() {
    let source = "69+320+3000";


    let parser = &mut Parser::init_parser(source);
    
    // while !parser.scanner.is_at_end() {
    //     parse_number(parser);
    // }
    parser.compile(); // warmup


}
