use crate::parser::parser::Parser;
use crate::parser::core::symbol::{create_new_symbol, get_symbol, set_symbol};
use crate::parser::expression::expression::expression;
use crate::parser::expression::expr::{DataType, Expression, ParseError};
use crate::scanner::token::TokenType;
use crate::parser::visitor::visitor::Accept;
use super::declaration::Declaration;
use crate::parser::visitor::rebuild_visitor::RebuildVisitor;
use crate::parser::visitor::print_visitor::PrintVisitor;

// evaluate an expression, then assign the expression at the location of the local variable with store
pub fn variable_assignment(parser: &mut Parser, var_name: &str) {
    let mut visitor = PrintVisitor;
    let mut rebuild = RebuildVisitor;
    expression(parser);

    if let Some(expr_ast) = parser.ast_stack.pop() {
        let expr = expr_ast.to_expression();
        let expr_datatype = expr.as_datatype();

        let test = Declaration::new_variable(var_name, Some(expr.clone()), expr_datatype.clone());
        parser.comment(&test.accept(&mut rebuild));
        parser.emit_instruction(&test.accept(&mut visitor));
        create_new_symbol(parser, var_name, expr_datatype);
        parser.print_symbols();
    }
}

// evaluate an expression with a specific expected type, then assign the expression
pub fn variable_assignment_with_type(parser: &mut Parser, var_name: &str, expected_type: DataType) {
    let mut visitor = PrintVisitor;
    let mut rebuild = RebuildVisitor;
    expression(parser);

    if let Some(expr_ast) = parser.ast_stack.pop() {
        let expr = expr_ast.to_expression();
        let expr_datatype = expr.as_datatype();

        // For arrays, update the size from the actual expression
        let final_type = match (&expected_type, &expr) {
            (DataType::Array(element_types, _), Expression::Array(array_expr)) => {
                DataType::Array(element_types.clone(), array_expr.size)
            }
            _ => expected_type.clone()
        };

        // TODO: Add type compatibility checking here
        let test = Declaration::new_variable(var_name, Some(expr.clone()), final_type.clone());
        parser.comment(&test.accept(&mut rebuild));
        parser.emit_instruction(&test.accept(&mut visitor));
        create_new_symbol(parser, var_name, final_type);
        parser.print_symbols();
    }
}
pub fn variable_declaration(parser: &mut Parser) {
    let global_name = parse_variable_name(parser, "Expected a variable name");
    parser.consume(
        TokenType::Colon, 
        "Expected : when declaring variable"
    );
    // Check if it's an array type [type] or simple type
    let type_tag = if parser.check_current(TokenType::LeftBracket) {
        // Array type syntax: [int], [bool], etc.
        parser.advance(); // consume '['

        parser.consume(
            TokenType::Identifier,
            "Expected element type identifier in array declaration"
        );

        let element_type_token = parser.previous.expect("Expected a token when getting the element type identifier");
        let element_type = match element_type_token.start {
            "int" => DataType::Integer(None),
            "bool" => DataType::Boolean(None),
            "str" => DataType::String("".to_string()),
            _ => {
                parser.error_at_previous("Unsupported array element type");
                DataType::Integer(None) // fallback
            }
        };

        parser.consume(
            TokenType::RightBracket,
            "Expected ']' after array element type"
        );

        // Create array type with default size (will be determined from initialization)
        DataType::Array(vec![element_type], 0)
    } else {
        // Simple type syntax: int, bool, str
        parser.consume(
            TokenType::Identifier,
            "Expected a type identifier when declaring variable"
        );
        let type_token = parser.previous.expect("Expected a token when getting the type identifier");
        match type_token.start {
            "int" => DataType::Integer(None),
            "bool" => DataType::Boolean(None),
            "str" => DataType::String("".to_string()),
            _ => {
                parser.error_at_previous("Unsupported variable type");
                DataType::Integer(None) // fallback
            }
        }
    };

    if parser.match_current(TokenType::Equal) {
        // Use type-aware assignment for arrays
        match &type_tag {
            DataType::Array(_, _) => variable_assignment_with_type(parser, &global_name, type_tag.clone()),
            _ => variable_assignment(parser, &global_name)
        }
    }
    else {
        let mut visitor = PrintVisitor;
        let test = Declaration::new_variable(
            &global_name, 
            None, 
            type_tag.clone()
        );
        parser.emit_instruction(&test.accept(&mut visitor));
        create_new_symbol(parser, &global_name, type_tag);
    }
    parser.expr_count += 1;
    parser.consume(TokenType::Semicolon, "Expected a semicolon after variable declaration");
}

pub fn parse_set_variable(parser: &mut Parser) {
    let identifier = parser.previous.expect("Expected a token when parsing the identifier when setting the variable");
    if parser.match_current(TokenType::Equal) {
        expression(parser);
        let expr = parser.ast_stack
            .pop()
            .expect("Expected an AstNode on the ast_stack")
            .to_expression()
            .clone();
        set_symbol(parser, identifier.start, expr);
        parser.consume(TokenType::Semicolon, "");
    } else {
        let _ = parse_get_variable(parser);
    }
}

#[allow(unused)]


pub fn parse_variable_name(parser: &mut Parser, err_msg: &str) -> String {
    parser.consume(TokenType::Identifier, err_msg);
    String::from(parser.previous.expect("Expected a variable name token when getting variable name").start)
}

pub fn parse_get_variable(parser: &mut Parser) -> Result<Expression, ParseError>{
    let value = parser.previous.expect("Tried to get previous token, but it was empty");
    get_symbol(parser, value.start);
    Ok(Expression::Empty)
}