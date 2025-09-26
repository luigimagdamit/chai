use crate::parser::expression::expr::ParseError;
use crate::parser::core::ast_node::AstNode;
use super::super::parser::Parser;
use super::super:: expression::expr::{DataType, Expr, Expression};
use crate::codegen::primitives_ir::{NumberIR, PrimitivesIR};
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

pub fn parse_number(parser: &mut Parser) -> Result<Expression, ParseError> {
    let value = String::from(parser.previous.expect("Tried to get previous token, but it was empty").start);
    let number_leaf = Expr {
        left: with_primitives_ir!(number_left(&value)),
        right: with_primitives_ir!(number_right(&value)),
        data_type: DataType::Integer(Some(value.parse().expect("Tried to convert a string into a number type")))
    };

    parser.constant_stack.push(Some(number_leaf.clone()));
    let expr_ast = Expression::Literal(number_leaf.data_type);
    parser.ast_stack.push(AstNode::from_expression(expr_ast.clone()));
    Ok(expr_ast)
}
