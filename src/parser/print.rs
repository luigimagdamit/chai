use {
    super::{
        parser::Parser,
        parse_fn::expression,
        expr::DataType
    },
    crate::{
        llvm::llvm_print::llvm_call_print_local,
        scanner::token::TokenType
    }
    
};


pub fn print_statement(parser: &mut Parser) {

    expression(parser);
    if let Some(expr) = parser.constant_stack.pop() {
        let value = expr.unwrap();
        let print_val = value.left;
        println!("%{} = add {}, 0", parser.expr_count , print_val);
        match value.data_type {
            DataType::Boolean(_) => llvm_call_print_local(parser.expr_count, "i1"),
            DataType::Integer(_) => llvm_call_print_local(parser.expr_count, "i32"),
            _ => ()
        }
    }
    
    parser.expr_count += 1;
    parser.consume(TokenType::Semicolon, "Expect semicolon after value");
}