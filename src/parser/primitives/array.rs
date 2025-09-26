use super::super::parser::Parser;
use crate::parser::expression::expr::{DataType, Expression, ParseError, ArrayExpression};
use crate::parser::core::ast_node::AstNode;
use crate::parser::expression::expression::parse_precedence;
use crate::parser::expression::precedence::Precedence;
use crate::scanner::token::TokenType;

/// Parse array literals like [1, 2, 3] or [true, false, true]
pub fn parse_array_literal(parser: &mut Parser) -> Result<Expression, ParseError> {
    // We're currently on the '[' token, advance to the first element
    parser.advance();

    let mut elements = Vec::new();
    let mut element_type = None;

    // Handle empty array case []
    if parser.check_current(TokenType::RightBracket) {
        parser.advance(); // consume ']'

        // For empty arrays, we need to infer type or default to int
        let default_type = DataType::Integer(None);
        let array_expr = ArrayExpression::empty(
            format!("arr_{}", parser.expr_count),
            default_type.clone(),
            0,
            parser.expr_count as usize
        );

        parser.expr_count += 1;
        let result = Expression::Array(array_expr);
        parser.ast_stack.push(AstNode::from_expression(result.clone()));

        return Ok(result);
    }

    // Parse array elements
    loop {
        // Parse the next expression
        parse_precedence(parser, Precedence::PrecAssignment);
        let ast_node = parser.ast_stack.pop().ok_or(ParseError::Generic)?;
        let expr_result = ast_node.to_expression();
        let expr_datatype = expr_result.as_datatype();

        // Check type consistency for the array
        match &element_type {
            None => {
                // First element determines the array type
                element_type = Some(expr_datatype.clone());
            }
            Some(expected_type) => {
                // Verify all elements have the same type
                if std::mem::discriminant(expected_type) != std::mem::discriminant(&expr_datatype) {
                    parser.error_at_previous("Array elements must have the same type");
                    return Err(ParseError::Generic);
                }
            }
        }

        elements.push(expr_result);

        // Check for comma or end of array
        if parser.check_current(TokenType::Comma) {
            parser.advance(); // consume ','

            // Allow trailing comma
            if parser.check_current(TokenType::RightBracket) {
                break;
            }
        } else if parser.check_current(TokenType::RightBracket) {
            break;
        } else {
            parser.error_at_previous("Expected ',' or ']' in array literal");
            return Err(ParseError::Generic);
        }
    }

    if !parser.check_current(TokenType::RightBracket) {
        parser.error_at_previous("Expected ']' after array elements");
        return Err(ParseError::Generic);
    }

    parser.advance(); // consume ']'

    // Create the array expression
    let array_size = elements.len();
    let element_datatype = element_type.unwrap_or(DataType::Integer(None));

    let array_expr = ArrayExpression::new(
        format!("arr_{}", parser.expr_count),
        elements,
        element_datatype,
        array_size,
        parser.expr_count as usize
    );

    // Generate LLVM IR for array allocation and initialization
    let element_type_str = match &array_expr.element_type {
        DataType::Integer(_) => "i32",
        DataType::Boolean(_) => "i1",
        DataType::String(_) => "i8*",
        DataType::Array(_, _) => "array", // nested arrays
    };

    // Emit array allocation instruction
    let alloca_instruction = format!("\t%{} = alloca [{}], align 16",
        parser.expr_count, array_size);
    parser.emit_instruction(&alloca_instruction);

    // Emit initialization instructions for each element
    for (index, element) in array_expr.elements.iter().enumerate() {
        // Generate element pointer
        let ptr_reg = parser.expr_count + 1000 + (index as u32);
        let ptr_instruction = format!("\t%{} = getelementptr inbounds [{}], [{}]* %{}, i64 0, i64 {}",
            ptr_reg, array_size, array_size, parser.expr_count, index);
        parser.emit_instruction(&ptr_instruction);

        // Store the element value
        let element_value = element.resolve_operand();
        let store_instruction = format!("\tstore {} {}, {}* %{}",
            element_type_str, element_value, element_type_str, ptr_reg);
        parser.emit_instruction(&store_instruction);
    }

    parser.expr_count += 1;

    let result = Expression::Array(array_expr);
    parser.ast_stack.push(AstNode::from_expression(result.clone()));

    Ok(result)
}

/// Parse array indexing like arr[0] or arr[index]
pub fn parse_array_index(parser: &mut Parser) -> Result<Expression, ParseError> {
    // The left side should be an array variable
    let array_expr = parser.ast_stack.pop().ok_or(ParseError::Generic)?;
    let array_expression = array_expr.to_expression();

    // Advance past the '['
    parser.advance();

    // Parse the index expression
    parse_precedence(parser, Precedence::PrecAssignment);
    let index_ast = parser.ast_stack.pop().ok_or(ParseError::Generic)?;
    let index_expr = index_ast.to_expression();

    // Expect ']'
    if !parser.check_current(TokenType::RightBracket) {
        parser.error_at_previous("Expected ']' after array index");
        return Err(ParseError::Generic);
    }
    parser.advance(); // consume ']'

    // Generate array indexing IR
    match &array_expression {
        Expression::Variable(var_expr) => {
            // Generate element pointer access
            let ptr_reg = parser.expr_count;
            let element_type_str = match &var_expr.datatype {
                DataType::Array(_, size) => {
                    // Generate getelementptr instruction
                    let ptr_instruction = format!("\t%{} = getelementptr inbounds [{}], [{}]* %{}, i64 0, {}",
                        ptr_reg, size, size, var_expr.name, index_expr.resolve_operand());
                    parser.emit_instruction(&ptr_instruction);
                    "i32" // For now, assume array elements are integers
                }
                _ => {
                    parser.error_at_previous("Cannot index non-array type");
                    return Err(ParseError::Generic);
                }
            };

            // Generate load instruction
            let load_reg = parser.expr_count + 1;
            let load_instruction = format!("\t%{} = load {}, {}* %{}",
                load_reg, element_type_str, element_type_str, ptr_reg);
            parser.emit_instruction(&load_instruction);

            parser.expr_count += 2;

            // Return a variable expression representing the loaded value
            let result_expr = Expression::from_literal(DataType::Integer(None));
            parser.ast_stack.push(AstNode::from_expression(result_expr.clone()));

            Ok(result_expr)
        }
        Expression::Array(array_expr) => {
            // Direct array access
            let element_type_str = match &array_expr.element_type {
                DataType::Integer(_) => "i32",
                DataType::Boolean(_) => "i1",
                DataType::String(_) => "i8*",
                _ => "i32"
            };

            // Generate element pointer access
            let ptr_reg = parser.expr_count;
            let ptr_instruction = format!("\t%{} = getelementptr inbounds [{}], [{}]* %{}, i64 0, {}",
                ptr_reg, array_expr.size, array_expr.size, array_expr.register, index_expr.resolve_operand());
            parser.emit_instruction(&ptr_instruction);

            // Generate load instruction
            let load_reg = parser.expr_count + 1;
            let load_instruction = format!("\t%{} = load {}, {}* %{}",
                load_reg, element_type_str, element_type_str, ptr_reg);
            parser.emit_instruction(&load_instruction);

            parser.expr_count += 2;

            let result_expr = Expression::from_literal(array_expr.element_type.clone());
            parser.ast_stack.push(AstNode::from_expression(result_expr.clone()));

            Ok(result_expr)
        }
        _ => {
            parser.error_at_previous("Cannot index this expression type");
            Err(ParseError::Generic)
        }
    }
}