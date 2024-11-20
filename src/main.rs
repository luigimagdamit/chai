use core::fmt;
use std::fmt::write;

#[derive(Clone)]
enum KeywordType {
    Print
}
#[derive(Clone)]
enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod
}
#[derive(Clone)]
enum Type {
    Number,
    String,
    Boolean,
    Operator(Op),
    Keyword(KeywordType),
    Identifiers
}
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Keyword(KeywordType::Print) => write!(f, " is Keyword -> Print"),
            Type::Operator(Op::Add) => write!(f, " is <Operator/Add>"),
            Type::Number => write!(f, " is Number"),
            _ => write!(f, "Unidentified <Type>")
        }
    }
}
impl Copy for Type {}
impl Copy for KeywordType {}
impl Copy for Op {}

struct Tree {
    left: Type,
    operator: Op,
    right: Type
}

struct Literal <'a>{
    lexeme: &'a str,
    literal_type: Type
}
impl<'a> fmt::Display for Literal <'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Literal]\n\t<&'a str> Lexeme: {}, \n\t<Type> literal_type {}", self.lexeme, self.literal_type)
    }
}

struct ParserSuccess <'a>{
    literal: Literal<'a>,
    remainder: &'a str
}
impl<'a> fmt::Display for ParserSuccess <'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ParserSuccess]\n<Literal> literal: {}, <&'a str> remainder {}", self.literal, self.remainder)
    }
}
fn match_literal<'a>(expected: &'static str, t: Type) 
    -> impl Fn(&'a str) -> Result<ParserSuccess, &'a str> {
    move |input| {
        match input.get(0..expected.len()) {
            Some(next) if next == expected => {
                // Create a Literal using a reference to part of the input string
                let literal = Literal {
                    lexeme: &input[0..expected.len()],
                    literal_type: t,
                };
                let success = ParserSuccess {
                    literal: literal,
                    remainder: &input[expected.len()..]
                };
                // Return the remaining string and the Literal struct
                Ok((success))
            }
            _ => Err(input), // No match, return the original input
        }
    }
}


fn main() {
    let PrintLn = match_literal("println", Type::Keyword(KeywordType::Print));
    let Add = match_literal("+", Type::Operator(Op::Add));
    
    let result = PrintLn("println(\"echo\")");
    //let result = parse_add("+");
    match result {
        Ok(_) => {
            let success = result.unwrap();
            
            println!("{} {}", success.literal, success.remainder)
        },
        Err(_) => {print!("Error")}
    };


}
