
use super::super::parser::Parser;
use crate::parser::expression::expr::{DataType, Expr, Expression, ParseError};
use crate::parser::core::ast_node::AstNode;
use crate::scanner::token::TokenType;
use crate::codegen::primitives_ir::{BooleanIR, PrimitivesIR};
use crate::codegen::llvm_primitives_ir::LlvmPrimitivesIR;
use crate::codegen::c_primitives_ir::CPrimitivesIR;
use crate::codegen::backend_config::{get_current_backend, IRBackend};

/// Macro to execute primitives IR-specific code based on current backend
macro_rules! with_primitives_ir {
    ($method:ident($($args:expr),*)) => {{
        match get_current_backend() {
            IRBackend::LLVM => {
                let ir = LlvmPrimitivesIR;
                ir.$method($($args),*)
            }
            IRBackend::C => {
                let ir = CPrimitivesIR;
                ir.$method($($args),*)
            }
        }
    }};
}
fn create_boolean(parser: &mut Parser, token_type: TokenType) {
    match token_type {
        TokenType::False => parser.new_expr(new_bool_val(false)),
        TokenType::True => parser.new_expr(new_bool_val(true)),
        _ => ()
    }
}
pub fn parse_literal(parser: &mut Parser) -> Result<Expression, ParseError> {
    if let Some(prev) = parser.previous {
        match prev.token_type {
            TokenType::False => {
                create_boolean(parser, TokenType::False);

                // Generate LLVM instruction to create register with boolean value
                let register_num = parser.expr_count;
                let instruction = format!("\t%{} = add i1 0, 0", register_num);
                parser.emit_instruction(&instruction);
                parser.expr_count += 1;

                let false_expr = Expression::from_literal(DataType::Boolean(Some(false)));
                parser.ast_stack.push(AstNode::from_expression(false_expr.clone()));

                Ok(false_expr)
            },
            TokenType::True => {
                create_boolean(parser, TokenType::True);

                // Generate LLVM instruction to create register with boolean value
                let register_num = parser.expr_count;
                let instruction = format!("\t%{} = add i1 0, 1", register_num);
                parser.emit_instruction(&instruction);
                parser.expr_count += 1;

                let true_expr = Expression::from_literal(DataType::Boolean(Some(true)));
                parser.ast_stack.push(AstNode::from_expression(true_expr.clone()));
                Ok(true_expr)
            },
            _ => {
                parser.error_at_previous(LITERAL_ERROR);
                Err(ParseError::Generic)
            },
        }
    } else {
        Err(ParseError::Generic)
    }
    
}

// Helper functions
fn new_bool_val(bool_val: bool) -> Expr {
    Expr {
        left: with_primitives_ir!(boolean_left(bool_val)),
        right: with_primitives_ir!(boolean_right(bool_val)),
        data_type: DataType::Boolean(Some(bool_val))
    }
}
const LITERAL_ERROR: &str = "Tried creating a new literal, but prev.token_type is not a True or False TokenType";