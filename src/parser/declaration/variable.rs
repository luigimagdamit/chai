use crate::llvm::llvm_string::llvm_retrieve_static_string;
use crate::parser::parser::{Parser, StringEntry};
use crate::parser::symbol::{create_new_symbol, get_symbol, set_symbol};
use crate::parser::parse_fn::convert_type_tag;
use crate::parser::expression::expression::expression;
use crate::parser::expression::expr::DataType;
use crate::scanner::token::TokenType;

// evaluate an expression, then assign the expression at the location of the local variable with store
pub fn variable_assignment(parser: &mut Parser, var_name: &str) {
    expression(parser);
    let (expr, _) = parser.expr_pop();
    match &expr.data_type {
        DataType::Boolean(_) => (),
        DataType::Integer(_) => {
            let codegen = format!("\tstore i32 %{}, i32* %{}\t\t\t; int variable assignment (variable.rs)\n", parser.expr_top() , var_name);
            parser.emit_instruction(&codegen);
            create_new_symbol(parser, var_name.to_string(), expr.data_type);
        },
        DataType::String(_) => {
            let tmp_register = LlvmTempRegister::StaticString(parser.expr_top());
            let store_codegen = tmp_register.store_in_alloca(var_name);
            parser.emit_instruction(&store_codegen);
                
            create_new_symbol(parser, var_name.to_string(), expr.data_type);
        }
    }
    

    
}
pub fn variable_declaration(parser: &mut Parser) {
    let global_name = parse_variable_name(parser, "Expected a variable name");
    parser.consume(TokenType::Colon, "Expected : when declaring variable");
    parser.consume(TokenType::Identifier, "Expected a type identifier when declaring variable");
    let type_tag = convert_type_tag(parser.previous.unwrap().start);
    let codegen = format!("\t%{} = {}", global_name, type_tag);

    parser.emit_instruction(&codegen);
    if parser.match_current(TokenType::Equal) { variable_assignment(parser, &global_name) } else {}

    parser.consume(TokenType::Semicolon, "Expected a semicolon after variable declaration");
}

pub fn parse_set_variable(parser: &mut Parser) {
    let identifier = parser.previous.unwrap();
    if parser.match_current(TokenType::Equal) {
        expression(parser);
        parser.consume(TokenType::Semicolon, "");

        let expr = parser.expr_pop();

        set_symbol(parser, String::from(identifier.start), expr.0);
        parser.emit_instruction(&"\t\t\t\t\t; parse_set_variable".to_string());
    } else {
        panic!("neeed a identigfier expression in parse rule");
        //expression(parser);
    }
    // parser.consume(TokenType::Equal, "Expected assignment");
}

#[allow(unused)]
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
                format!("\tstore i8* %{register}, i8** %{target}\t\t\t\t ; storing item in a stack variable\n")
            },
            LlvmTempRegister::Number => panic!()
        }
    }
}

pub fn parse_variable_name(parser: &mut Parser, err_msg: &str) -> String {
    parser.consume(TokenType::Identifier, err_msg);
    String::from(parser.previous.unwrap().start)
}

pub fn parse_get_variable(parser: &mut Parser) {
    let value = parser.previous.unwrap();
    get_symbol(parser, String::from(value.start));
}