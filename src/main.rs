mod token;      // Importing the token module
mod parser;     // Importing the parser module
mod error;      // Importing the error module


use parser::parse_binary;

fn main() {
    let res = parse_binary("69+420))))");
    println!("{}", res.unwrap());

}
