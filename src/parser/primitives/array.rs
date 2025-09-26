use super::super::parser::Parser;
use crate::parser::expression::expr::{DataType, Expression, ParseError, ArrayExpression, VariableExpression};
use crate::parser::core::ast_node::AstNode;
use crate::parser::declaration::variable::parse_get_variable;
use crate::scanner::token::TokenType;

/// Parse array literals like [1, 2, 3] or [true, false, true]
pub fn parse_array_literal(parser: &mut Parser) -> Result<Expression, ParseError> {
    // The precedence parser has already advanced past the '[' token,
    // so we're now positioned at the first element

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
        // Parse individual array element based on token type
        let expr_result = match parser.current {
            Some(token) => {
                match token.token_type {
                    TokenType::Number => {
                        parser.advance();
                        let number_str = parser.previous.unwrap().start;
                        let number_val: i32 = number_str.parse().unwrap_or(0);
                        Expression::from_literal(DataType::Integer(Some(number_val)))
                    }
                    TokenType::True => {
                        parser.advance();
                        Expression::from_literal(DataType::Boolean(Some(true)))
                    }
                    TokenType::False => {
                        parser.advance();
                        Expression::from_literal(DataType::Boolean(Some(false)))
                    }
                    TokenType::String => {
                        parser.advance();
                        let string_val = parser.previous.unwrap().start.to_string();
                        Expression::from_literal(DataType::String(string_val))
                    }
                    TokenType::Identifier => {
                        // Variable reference - use the existing function
                        parse_get_variable(parser)?
                    }
                    _ => {
                        parser.error_at_previous("Unsupported array element type");
                        return Err(ParseError::Generic);
                    }
                }
            }
            None => {
                parser.error_at_previous("Unexpected end of input in array literal");
                return Err(ParseError::Generic);
            }
        };

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
    let alloca_instruction = format!("\t%{} = alloca [{} x {}], align 16",
        parser.expr_count, array_size, element_type_str);
    parser.emit_instruction(&alloca_instruction);

    // Emit initialization instructions for each element
    for (index, element) in array_expr.elements.iter().enumerate() {
        // Generate element pointer
        let ptr_reg = parser.expr_count + 1000 + (index as u32);
        let ptr_instruction = format!("\t%{} = getelementptr inbounds [{} x {}], [{} x {}]* %{}, i64 0, i64 {}",
            ptr_reg, array_size, element_type_str, array_size, element_type_str, parser.expr_count, index);
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
    println!("DEBUG: parse_array_index called!");
    println!("DEBUG: Current token: {:?}", parser.current);
    println!("DEBUG: Previous token: {:?}", parser.previous);

    // The left side should be an array variable
    println!("DEBUG: AST stack size before pop: {}", parser.ast_stack.len());
    let array_expr = parser.ast_stack.pop().ok_or(ParseError::Generic)?;
    let array_expression = array_expr.to_expression();

    println!("DEBUG: Found array expression");

    // The '[' token has already been consumed by the precedence parser
    // We are now positioned at the index expression

    // Parse the index expression - simple number for now
    if !parser.check_current(TokenType::Number) {
        println!("DEBUG: Expected number but found: {:?}", parser.current);
        parser.error_at_previous("Array index must be a number");
        return Err(ParseError::Generic);
    }

    parser.advance();
    let index_str = parser.previous.unwrap().start;
    let index_val: i32 = index_str.parse().unwrap_or(0);
    let index_expr = Expression::from_literal(DataType::Integer(Some(index_val)));

    // Expect ']'
    if !parser.check_current(TokenType::RightBracket) {
        parser.error_at_previous("Expected ']' after array index");
        return Err(ParseError::Generic);
    }
    parser.advance(); // consume ']'

    // Generate array indexing IR
    println!("DEBUG: About to match array expression type");
    match &array_expression {
        Expression::Variable(var_expr) => {
            // Generate element pointer access
            // Use a high register number to avoid conflicts with array literal registers
            let ptr_reg = parser.expr_count + 1010;
            println!("DEBUG: Array indexing ptr_reg: {}, expr_count: {}", ptr_reg, parser.expr_count);
            let element_type_str = match &var_expr.datatype {
                DataType::Array(_, size) => {
                    // Generate getelementptr instruction
                    let element_type = "i32"; // For now, assume array elements are integers
                    let ptr_instruction = format!("\t%{} = getelementptr inbounds [{} x {}], [{} x {}]* %{}, i64 0, i64 {}",
                        ptr_reg, size, element_type, size, element_type, var_expr.name, index_expr.resolve_operand());
                    parser.emit_instruction(&ptr_instruction);
                    element_type
                }
                _ => {
                    parser.error_at_previous("Cannot index non-array type");
                    return Err(ParseError::Generic);
                }
            };

            // Generate load instruction
            let load_reg = ptr_reg + 1;
            let load_instruction = format!("\t%{} = load {}, {}* %{}",
                load_reg, element_type_str, element_type_str, ptr_reg);
            parser.emit_instruction(&load_instruction);

            parser.expr_count += 2;

            // Return a variable expression representing the loaded value
            // For register references, we need a way to get just %{load_reg}
            // Since VariableExpression::get_value() returns %{name}_{count},
            // and we want %{load_reg}, we can create a custom variable with name that already includes %
            let result_expr = Expression::Variable(VariableExpression {
                name: format!("{}", load_reg), // Just the register number
                datatype: DataType::Integer(None),
                count: 0  // This will make get_value() return %{load_reg}_0
            });
            parser.ast_stack.push(AstNode::from_expression(result_expr.clone()));

            println!("DEBUG: parse_array_index Variable case returning successfully");
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
            let ptr_reg = parser.expr_count + 1010;
            let ptr_instruction = format!("\t%{} = getelementptr inbounds [{} x {}], [{} x {}]* %{}, i64 0, i64 {}",
                ptr_reg, array_expr.size, element_type_str, array_expr.size, element_type_str, array_expr.register, index_expr.resolve_operand());
            parser.emit_instruction(&ptr_instruction);

            // Generate load instruction
            let load_reg = ptr_reg + 1;
            let load_instruction = format!("\t%{} = load {}, {}* %{}",
                load_reg, element_type_str, element_type_str, ptr_reg);
            parser.emit_instruction(&load_instruction);

            parser.expr_count += 2;

            let result_expr = Expression::Variable(VariableExpression {
                name: load_reg.to_string(),
                datatype: array_expr.element_type.clone(),
                count: 0
            });
            parser.ast_stack.push(AstNode::from_expression(result_expr.clone()));

            println!("DEBUG: parse_array_index returning successfully");
            Ok(result_expr)
        }
        _ => {
            parser.error_at_previous("Cannot index this expression type");
            Err(ParseError::Generic)
        }
    }
}