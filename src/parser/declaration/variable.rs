use crate::llvm::llvm_string::llvm_retrieve_static_string;
use crate::parser::parser::{Parser, StringEntry};
use crate::parser::symbol::{create_new_symbol, get_symbol, set_symbol};
use crate::parser::parse_fn::{expression, convert_type_tag};
use crate::parser::expression::expr::DataType;
use crate::{common::flags::PARSE_DECLARATION_MODE, scanner::token::TokenType};

// misleading title, will just 
pub fn parse_variable_name(parser: &mut Parser, err_msg: &str) -> String {
    parser.consume(TokenType::Identifier, err_msg);
    String::from(parser.previous.unwrap().start)
    // store this in the hash table
}

pub fn parse_get_variable(parser: &mut Parser) {
    let value = parser.previous.unwrap();
    get_symbol(parser, String::from(value.start));
}

pub enum LlvmTempRegister {
    StaticString(u32), // holds string value for lookup
    Number
}
impl LlvmTempRegister {
    pub fn new_register(&self, lookup: &StringEntry) -> String {
        match self {
            Self::StaticString(register) => {
                format!("\t%{} = {}\t\t;LLVM Register for String @ ExprCount {}(variable.rs) ", register, llvm_retrieve_static_string(lookup.length, lookup.index), register)
            },
            LlvmTempRegister::Number => {
                panic!()
            }
        }
    }
    pub fn store_in_alloca(&self, target: &str) -> String {
        match self {
            Self::StaticString(register) => {
                format!("\tstore i8* %{register}, i8** %{target}")
            },
            LlvmTempRegister::Number => panic!()
        }
    }
}
// evaluate an expression, then assign the expression at the location of the local variable with store
pub fn variable_assignment(parser: &mut Parser, var_name: &str) {
    expression(parser);
    if let Some(expr) = parser.constant_stack.pop() {
        let value = expr.unwrap_or_else(||panic!("Tried evaluation an expression in print_statement, but opened an empty Expr"));
        let print_val = &value.left;
        parser.expr_count += 1;
        match &value.data_type {
            DataType::Boolean(_) => (),
            DataType::Integer(int) => {
                let codegen = format!("\tstore i32 {}, i32* %{}\t\t\t; int variable assignment (variable.rs)\n", int , var_name);
                println!("{}", codegen);
                parser.emit_instruction(&codegen);
                create_new_symbol(parser, String::from(var_name), value.data_type);
            },
            DataType::String(str_value) => {
                // pop off the stack
                let str_lookup = parser.string_table.get(str_value).clone();
                if let Some(lookup_result) = str_lookup {
                    
                    // look up the string value within the Expr, do lookup, then get the constant index
                    // within StringEntry in order to retrieve appropariate value in LLVM Constants

                    // %2 = getelementptr inbounds [13 x i8], [13 x i8]* @str0, i32 0, i32 0
                    let tmp_register = LlvmTempRegister::StaticString(parser.expr_count);
                    let load_string_codegen = tmp_register.new_register(lookup_result);
                    println!("{}", load_string_codegen);
                    parser.emit_instruction(&load_string_codegen);

                    // store i8* %2, i8** %b
                    let store_codegen = tmp_register.store_in_alloca(var_name);
                    println!("{}", store_codegen);
                    parser.emit_instruction(&store_codegen);
                }


                

                create_new_symbol(parser, String::from(var_name), value.data_type);
                parser.expr_count += 1; // we used a tmp register
            }
        }
    }
    

    
}
pub fn variable_declaration(parser: &mut Parser) {
    // let name: type;
    let global_name = parse_variable_name(parser, "Expected a variable name");
    parser.consume(TokenType::Colon, "Expected : when declaring variable");
    parser.consume(TokenType::Identifier, "Expected a type identifier when declaring variable");
    let type_tag = convert_type_tag(parser.previous.unwrap().start);
    let codegen = format!("\t%{} = {}", global_name, type_tag);
    if PARSE_DECLARATION_MODE { println!("{}", codegen) }
    parser.compilation += &codegen;
    if parser.match_current(TokenType::Equal) {

        variable_assignment(parser, &global_name);
        
    } else {
        
    }


    parser.consume(TokenType::Semicolon, "Expected a semicolon after variable declaration");
}

pub fn parse_set_variable(parser: &mut Parser) {
    let identifier = parser.previous.unwrap();
    parser.consume(TokenType::Equal, "Expected assignment");
    expression(parser);
    parser.consume(TokenType::Semicolon, "");
    
    if let Some(expr) = parser.constant_stack.pop() {
        match expr {
            Some(new_value) => set_symbol(parser, String::from(identifier.start), new_value),
            None => parser.error_at_previous("Expected an <expression> when setting variable to a new value"),
        }
    } else {

        parser.error_at(&identifier, "Unknown variable (set_variable)");
    }



}