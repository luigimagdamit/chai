use crate::parser::expression::expr::DataType;
use crate::parser::parse_fn::expression;
use crate::parser::parser::Parser;
use crate::{common::common::PARSE_DECLARATION_MODE, llvm::llvm_print::llvm_call_print_local, scanner::token::TokenType};



pub fn print_statement(parser: &mut Parser) {

    expression(parser);
    if let Some(expr) = parser.constant_stack.pop() {
        let value = expr.unwrap_or_else(||panic!("Tried evaluation an expression in print_statement, but opened an empty Expr"));
        let print_val = &value.left;
        
        match &value.data_type {
            DataType::Boolean(_) => {
                llvm_print_i1_local(parser.expr_count, print_val);
            }
            DataType::Integer(_) => {
                let codegen = llvm_print_i32_local(parser.expr_count, print_val);
                parser.emitInstruction(&codegen);

            },
            DataType::String(_) => {
                if value.right != "<__var_string__>" {
                    
                    parser.compilation += &llvm_print_str_local(parser.expr_count, print_val);
                    // does not place anything on the stack...
                    //parser.expr_count += 1;
                } else {
                    let codegen = format!("call i32 (i8*, ...) @printf(i8* {})", print_val);
                    if PARSE_DECLARATION_MODE { println!("{}", codegen)}
                    parser.emitInstruction(&codegen);
                    //parser.expr_count += 1;
                }
                
            }
        }
    }
    
    parser.expr_count += 1;
    parser.consume(TokenType::Semicolon, "Expect semicolon after value");
}

fn llvm_print_i32_local(reg_name: u32, value: &String) -> String{
    let c1 = format!("%{} = add {}, 0\n", reg_name , value);
    println!("{}", c1);
    let c2 = llvm_call_print_local(reg_name, "i32");
    println!("{}", c2);
    c1 + &c2
}
fn llvm_print_str_local(reg_name: u32, value: &String) -> String {
    let c1 = format!("\n%{} = {}\n", reg_name, value);
    let c2 = format!("call i32 (i8*, ...) @printf(i8* %{})\n", reg_name);
    let res = String::from(c1 + &c2);

    if PARSE_DECLARATION_MODE { println!("{}", &res)}
    res
}
fn llvm_print_i1_local(reg_name: u32, value: &String) {
    println!("%{} = add {}, 0", reg_name , value);
    llvm_call_print_local(reg_name, "i1");
}