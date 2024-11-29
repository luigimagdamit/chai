
use crate::parser::parser::Parser;
use crate::parser::parse_rule::get_rule;
use crate::parser::precedence::Precedence;
use crate::parser::expr::{Expr, DataType};
use crate::scanner::token::TokenType;
use crate::common::common::{PARSE_CONSTANT_FOLD, PARSE_FN_OUTPUT};
use crate::llvm::llvm_primitives::llvm_top_level_expr;
use crate::llvm::llvm_print::{llvm_call_print, llvm_call_print_local, llvm_fmt_string_int, llvm_main_close, llvm_main_start, llvm_print_define, llvm_print_i32_define, llvm_print_no_main};

pub fn top_level_expr(parser: &mut Parser) {
    if let Some(constant) = parser.constant_stack.pop() {
        // println!("{}", &constant.unwrap().left);
        let expr_eval = &constant.unwrap();
        llvm_top_level_expr(&expr_eval.right, &expr_eval.data_type, 0);
    }
}
pub fn expression(parser: &mut Parser) {
    parse_precedence(parser, Precedence::PrecAssignment);

    if let Some(eof) = parser.current {
        match eof.token_type {
            TokenType::EOF => {
                // parser.print_parser();
    
                // println!("{}", llvm_print_define());
                // llvm_fmt_string_int();
                // llvm_print_i32_define();
                
                // top_level_expr(parser);
                // llvm_main_start();
                // llvm_call_print(0, "i32");
                // llvm_main_close();
                
            },
            _ => {}
        }
        
    }
    // assume this is just a high level expression


    
    
    
}
// fn name() ret type
pub fn parse_fn_declare(parser: &mut Parser) {
    print!("\ndefine ");
    parser.consume(TokenType::Identifier, "Expected function name");
    let fn_name = parser.previous.unwrap().clone();
    
    parser.consume(TokenType::LeftParen, "");
    
    parser.consume(TokenType::RightParen, "");
    parser.consume(TokenType::Identifier, "");
    let fn_type = parser.previous.unwrap_or_else(|| panic!()).clone();
    parser.consume(TokenType::LeftBrace, "Expected {");


    // i32
    
    
    print!("{}", fn_type.start);
    print!(" @{}", fn_name.start);
    print!("(");
    // func args here
    print!("){{");
    println!("\nentry:");
    // func body here
    //
    expression(parser);
    llvm_call_print_local(parser.expr_count-1, "i32");
    parser.consume(TokenType::RightBrace, "Unclosed function body");
    println!("ret i32 0\n}}");

    
}
pub fn parse_number(parser: &mut Parser) {
    let value = parser.previous.unwrap().start;
    let number_leaf = Expr {
        left: String::from(format!("i32 {}", value)),
        right: String::from(value),
        data_type: DataType::Integer(value.parse().unwrap())
    };

    parser.constant_stack.push(Some(number_leaf));
}
fn create_literal(parser: &mut Parser, token_type: TokenType, value: &str) {
    match token_type {
        TokenType::False => {
            let false_leaf = Expr {
                left: String::from(format!("i1 {}", value)),
                right: String::from(value),
                data_type: DataType::Boolean(false)
            };
            false_leaf.print_leaf();
            parser.constant_stack.push(Some(false_leaf));
        }
        TokenType::True => {
            let false_leaf = Expr {
                left: String::from(format!("i1 {}", value)),
                right: String::from(value),
                data_type: DataType::Boolean(true)
            };
            false_leaf.print_leaf();
            parser.constant_stack.push(Some(false_leaf));
        }
        _ => parser.error_at(&parser.previous.unwrap(), "Invalid literal token"),
    }
}
pub fn parse_literal(parser: &mut Parser) {
    if let Some(prev) = parser.previous {
        match prev.token_type {
            TokenType::False => create_literal(parser, TokenType::False, "0"),
            TokenType::True => create_literal(parser, TokenType::True, "1"),
            _ => parser.error_at(&prev, "Unrecognizeed literal token type"),
        }
    }
}
pub fn parse_precedence(parser: &mut Parser, precedence: Precedence) {
    parser.advance();
    if let Some(prev) = parser.previous {
        if let Some(prefix_fn) = get_rule(prev.token_type).prefix {
            prefix_fn(parser);
        } else {
            let err_msg = format!("Expected a prefix rule for token <{}>", prev);
            parser.error_at(&prev, &err_msg);
        }

        if let Some(_) = parser.current {
            while precedence.to_u32() <= get_rule(parser.current.unwrap().token_type).precedence.to_u32() {
                parser.advance();
                if let Some(infix_rule) = get_rule(parser.previous.unwrap().token_type).infix {
                    infix_rule(parser);
                } else {
                    break
                }
                parser.print_parser();
            }
        }
    }
    
}

pub fn parse_binary(parser: &mut Parser) {
    if let Some(token) = parser.previous {
       
        let operator_type = token.token_type;
        let rule_fn = get_rule(operator_type);
        let new_prec = rule_fn.precedence as u32;
        parse_precedence(parser, Precedence::from_u32(new_prec + 1));

        match operator_type {
            TokenType::Plus => {
                if PARSE_FN_OUTPUT { println!("<add>"); }
                print!("%{} = add ", parser.expr_count);
                
                binary_op(parser, add_op);
                parser.expr_count += 1;
                
            },
            TokenType::Minus => {
                if PARSE_FN_OUTPUT { println!("<minus>"); }
                binary_op(parser, sub_op);
            },
            TokenType::Star => {
                if PARSE_FN_OUTPUT { println!("<multiply>"); }
                binary_op(parser, mult_op);
            }
            TokenType::Slash => {
                if PARSE_FN_OUTPUT { println!("<divide>"); }
                binary_op(parser, div_op);
            }
            _ => {}
        }
        


    }
    
}

pub fn parse_grouping(parser: &mut Parser) {

    expression(parser);
    //parser.constant_stack.pop().unwrap().unwrap().print_leaf();
    parser.consume(TokenType::RightParen, "Expect ')' after expression, found something else at");
}

fn add_op(a: i32, b: i32) -> i32 {
    a + b
}
fn sub_op(a: i32, b: i32) -> i32 {
    a - b
}
fn mult_op(a: i32, b: i32) -> i32 {
    a * b
}
fn div_op(a: i32, b: i32) -> i32 {
    a / b
}
fn binary_op(parser: &mut Parser, operator: fn(i32, i32) -> i32) 
where

{
    let local_right = &mut parser.constant_stack.pop().unwrap_or_else(|| panic!());
    let local_left = &mut parser.constant_stack.pop().unwrap_or_else(|| panic!());
    
    let left = local_left.clone().unwrap();
    let right = local_right.clone().unwrap();
    print!("{}, ", left.left);
    print!("{}\n", right.right);
    
    match (left.data_type, right.data_type) {
        (DataType::Integer(a), DataType::Integer(b)) => {
            let calculation = operator(a, b);
            if PARSE_CONSTANT_FOLD {
                parser.constant_stack.push(Some(Expr {
                left: String::from("i32 ") + &(calculation).to_string(),
                right: (calculation).to_string(),
                data_type: DataType::Integer(calculation) 
                }))
            } else {
                parser.constant_stack.push(Some(Expr {
                    left: String::from("i32 %") + &(parser.expr_count).to_string(),
                    right: (parser.expr_count).to_string(),
                    data_type: DataType::Integer(calculation) 
                }))
            }
            // println!("<add: <constant fold: {}+{}={}>>", a, b, a + b);


            
        }
        (_, _) => {
            if let Some(t) = parser.previous {
                parser.error_at(&t, "Invalid binary operands");
            }
        },
        // _ => println!("<left operand: {}> <plus> <right operand: {}>", left.left, right.right)
    }
    // if PARSE_FN_OUTPUT {
    //     parser.print_parser();
    // }
}
