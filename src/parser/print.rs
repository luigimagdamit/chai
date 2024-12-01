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
        let value = expr.unwrap_or_else(||panic!("Tried evaluation an expression in print_statement, but opened an empty Expr"));
        let print_val = &value.left;
        
        match &value.data_type {
            DataType::Boolean(_) => llvm_call_print_local(parser.expr_count - 1, "i1"),
            DataType::Integer(_) => llvm_print_i32_local(parser.expr_count, print_val),
            DataType::String(_) => {
                if value.right != "__var_string" {
                    llvm_print_str_local(parser.expr_count, print_val);
                    parser.expr_count += 1;
                } else {
                    println!("call i32 (i8*, ...) @printf(i8* {})", print_val);
                    parser.expr_count += 1;
                }
                
            }
        }
    }
    
    parser.expr_count += 1;
    parser.consume(TokenType::Semicolon, "Expect semicolon after value");
}

fn llvm_print_i32_local(reg_name: u32, value: &String) {
    println!("%{} = add {}, 0", reg_name , value);
    llvm_call_print_local(reg_name, "i32")
}
fn llvm_print_str_local(reg_name: u32, value: &String) {
    println!("%{} = {}", reg_name, value);
    println!("call i32 (i8*, ...) @printf(i8* %{})", reg_name);
}