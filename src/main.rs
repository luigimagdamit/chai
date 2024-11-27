mod token;    // Import the Token and TokenType module
mod scanner;  // Import the Scanner module
mod parser;
mod error; 

use parser::Parser;



fn main() {
    let source = "69+320+3000";


    let parser = &mut Parser::init_parser(source);
    
    // while !parser.scanner.is_at_end() {
    //     parse_number(parser);
    // }
    parser.compile(); // warmup


}
