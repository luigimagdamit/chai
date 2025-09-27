use crate::parser::parser::Parser;
use crate::parser::core::symbol::{create_new_symbol, get_symbol, set_symbol};
use crate::parser::expression::expression::expression;
use crate::parser::expression::expr::{DataType, Expression, ParseError};
use crate::scanner::token::TokenType;
use crate::parser::visitor::visitor::Accept;
use super::declaration::Declaration;
use crate::parser::visitor::rebuild_visitor::RebuildVisitor;
use crate::parser::visitor::print_visitor::PrintVisitor;

fn parse_type_annotation(parser: &mut Parser) -> DataType {
    // Check if it's an array type [type] or simple type
    if parser.check_current(TokenType::LeftBracket) {
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

        // For now, create an array with unknown size (we'll determine it from the assignment)
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
    }
}

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

pub fn variable_assignment_with_inference(parser: &mut Parser, var_name: &str) {
    use crate::parser::expression::expr::Expression;

    // Parse the expression to determine the type
    expression(parser);

    if let Some(ast_node) = parser.ast_stack.pop() {
        let expr = ast_node.to_expression();
        let inferred_type = expr.as_datatype();

        println!("DEBUG: Variable assignment - expression type: {:?}", std::mem::discriminant(&expr));
        match &expr {
            Expression::Array(_) => println!("DEBUG: Found Expression::Array!"),
            Expression::Literal(_) => println!("DEBUG: Found Expression::Literal"),
            Expression::Variable(_) => println!("DEBUG: Found Expression::Variable"),
            Expression::Binary(_) => println!("DEBUG: Found Expression::Binary"),
            Expression::StringConstant(_) => println!("DEBUG: Found Expression::StringConstant"),
            Expression::TempRegister(_) => println!("DEBUG: Found Expression::TempRegister"),
            Expression::Empty => println!("DEBUG: Found Expression::Empty"),
        }

        // Special handling for TempRegister and Array expressions
        match &expr {
            Expression::TempRegister(temp_reg) => {
                // For temp registers, allocate a proper variable and store the register value
                parser.comment(&format!("var {} = %{} (from temp register)", var_name, temp_reg.register));

                // Allocate variable storage
                let type_str = inferred_type.as_str();
                parser.emit_instruction(&format!("\t%{} = alloca {}, align 4", var_name, type_str));

                // Store the temp register value into the variable
                parser.emit_instruction(&format!("\tstore {} %{}, {}* %{}", type_str, temp_reg.register, type_str, var_name));

                // Store in symbol table
                create_new_symbol(parser, var_name, inferred_type);
            }
            Expression::Array(array_expr) => {
                // For arrays, copy the initialized array to the variable
                parser.comment(&format!("var {} : array[{}] = array[{}]:arr_{};", var_name, array_expr.size, array_expr.size, array_expr.register));
                println!("DEBUG: Array assignment detected! register = {}", array_expr.register);

                // Allocate space for the array variable
                parser.emit_instruction(&format!("\t%{} = alloca [{} x i32], align 16", var_name, array_expr.size));

                // Use LLVM memcpy to copy the entire array at once
                let size_bytes = array_expr.size * 4; // 4 bytes per i32
                parser.emit_instruction(&format!("\t%{}_src = bitcast [{} x i32]* %{} to i8*", var_name, array_expr.size, array_expr.register));
                parser.emit_instruction(&format!("\t%{}_dst = bitcast [{} x i32]* %{} to i8*", var_name, array_expr.size, var_name));
                parser.emit_instruction(&format!("\tcall void @llvm.memcpy.p0i8.p0i8.i64(i8* %{}_dst, i8* %{}_src, i64 {}, i1 false)", var_name, var_name, size_bytes));
                println!("DEBUG: Generated memcpy from %{} to %{}", array_expr.register, var_name);

                // Store in symbol table
                create_new_symbol(parser, var_name, inferred_type);
            }
            _ => {
                // Normal variable declaration for other expression types
                let decl = Declaration::new_variable(var_name, Some(expr), inferred_type.clone());
                let mut visitor = PrintVisitor;
                decl.accept(&mut visitor);

                // Store the variable in the symbol table
                create_new_symbol(parser, var_name, inferred_type);
            }
        }
    } else {
        parser.error_at_previous("Expected expression for variable assignment");
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

        // Special handling for array expressions to copy values correctly
        match &expr {
            Expression::Array(array_expr) => {
                println!("DEBUG: Found array assignment in variable_assignment_with_type! register = {}", array_expr.register);

                // For arrays, copy the initialized array to the variable
                parser.comment(&format!("var {} : array[{}] = array[{}]:arr_{};", var_name, array_expr.size, array_expr.size, array_expr.register));

                // Get the correct element type string and size
                let (element_type_str, element_size) = match &array_expr.element_type {
                    DataType::Integer(_) => ("i32", 4),
                    DataType::Boolean(_) => ("i1", 1),
                    DataType::String(_) => ("i8*", 8),
                    _ => ("i32", 4) // fallback
                };

                // Allocate space for the array variable with correct type
                parser.emit_instruction(&format!("\t%{} = alloca [{} x {}], align 16", var_name, array_expr.size, element_type_str));

                // Use LLVM memcpy to copy the entire array at once
                let size_bytes = array_expr.size * element_size;
                parser.emit_instruction(&format!("\t%{}_src = bitcast [{} x {}]* %{} to i8*", var_name, array_expr.size, element_type_str, array_expr.register));
                parser.emit_instruction(&format!("\t%{}_dst = bitcast [{} x {}]* %{} to i8*", var_name, array_expr.size, element_type_str, var_name));
                parser.emit_instruction(&format!("\tcall void @llvm.memcpy.p0i8.p0i8.i64(i8* %{}_dst, i8* %{}_src, i64 {}, i1 false)", var_name, var_name, size_bytes));
                println!("DEBUG: Generated memcpy from %{} to %{}", array_expr.register, var_name);

                // Store in symbol table
                create_new_symbol(parser, var_name, final_type);
            }
            _ => {
                // Normal variable declaration for other expression types
                let test = Declaration::new_variable(var_name, Some(expr.clone()), final_type.clone());
                parser.comment(&test.accept(&mut rebuild));
                parser.emit_instruction(&test.accept(&mut visitor));
                create_new_symbol(parser, var_name, final_type);
            }
        }
        parser.print_symbols();
    }
}
pub fn variable_declaration(parser: &mut Parser) {
    let global_name = parse_variable_name(parser, "Expected a variable name");

    // Check if there's a type annotation (colon) or direct assignment (equals)
    let type_tag = if parser.check_current(TokenType::Colon) {
        parser.advance(); // consume ':'
        Some(parse_type_annotation(parser))
    } else if parser.check_current(TokenType::Equal) {
        // Type inference - we'll determine the type from the expression
        None
    } else {
        parser.error_at_previous("Expected ':' for type annotation or '=' for assignment");
        return;
    };

    if parser.match_current(TokenType::Equal) {
        match type_tag {
            Some(explicit_type) => {
                // Use explicit type
                match &explicit_type {
                    DataType::Array(_, _) => variable_assignment_with_type(parser, &global_name, explicit_type),
                    _ => variable_assignment(parser, &global_name)
                }
            }
            None => {
                // Type inference from expression
                variable_assignment_with_inference(parser, &global_name)
            }
        }
    } else {
        // Declaration without assignment
        match type_tag {
            Some(explicit_type) => {
                let decl = Declaration::new_variable(&global_name, None, explicit_type);
                let mut visitor = PrintVisitor;
                decl.accept(&mut visitor);
            }
            None => {
                parser.error_at_previous("Variable declaration requires either type annotation or assignment");
            }
        }
    }

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
    println!("DEBUG: parse_get_variable called for variable: {}", value.start);
    get_symbol(parser, value.start);

    // get_symbol puts the variable expression on the stack
    // For prefix parsing, we should leave it on the stack for infix parsers to consume
    println!("DEBUG: AST stack size in parse_get_variable: {}", parser.ast_stack.len());
    if let Some(ast_node) = parser.ast_stack.last() {
        println!("DEBUG: Successfully found variable expression, leaving on stack");
        Ok(ast_node.clone().to_expression())
    } else {
        println!("DEBUG: Failed to find variable expression");
        Err(ParseError::Generic)
    }
}