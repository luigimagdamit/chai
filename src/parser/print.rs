use {
    super::{
        parser::Parser,
        parse_fn::expression
    },
    crate::{
        llvm::llvm_print::llvm_call_print_local,
        scanner::token::TokenType
    }
    
};


pub fn print_statement(parser: &mut Parser) {

    expression(parser);
    if let Some(expr) = parser.constant_stack.pop() {

        let print_val = expr.unwrap().left;
        println!("%{} = add {}, 0", parser.expr_count , print_val);
    }
    llvm_call_print_local(parser.expr_count, "i32");
    parser.expr_count += 1;
    parser.consume(TokenType::Semicolon, "Expect semicolon after value");
}