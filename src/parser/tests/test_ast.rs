
// This test is for testing the proper AST expression structure

mod tests {
    #![allow(unused_imports)]
    use core::panic;

    use crate::common::flags::PARSE_CONSTANT_FOLD;
    use crate::parser::expression::expr::{Binary, DataType, Expression, Operation};
    use crate::parser::expression::expression::parse_precedence;
    use crate::scanner::token::TokenType;
    use crate::parser::parser::Parser;
    use crate::parser::expression::precedence::Precedence;
    #[test] 
    fn test_parse_one_plus_two() {
        let parser = &mut Parser::init_parser("1+2\0");
        parser.advance();
        parse_precedence(parser, Precedence::PrecAssignment);

        if let Some(ast_node) = parser.ast_stack.pop() {
            let expr = ast_node.to_expression();
            let binary = Binary::from(expr.clone());
            let operands = binary_operands(binary.clone());

            // 1 - The operation is the correct type
            assert_eq!(expr.as_datatype(), DataType::Integer(0));

            // 2 - The operator is the correct binary operator
            assert_eq!(Binary::from(expr.clone()).operator, Operation::Add);
            
            // 3 - Test the operands
            test_operand_value_int(operands.0, 1);
            test_operand_value_int(operands.1, 2);
        }
        
    }
    #[test]
    fn test_parse_one_times_two() {
        let parser = &mut Parser::init_parser("1*2\0");
        parser.advance();
        parse_precedence(parser, Precedence::PrecAssignment);

        if let Some(ast_node) = parser.ast_stack.pop() {
            let expr = ast_node.to_expression();
            let binary = Binary::from(expr.clone());
            let operands = binary_operands(binary.clone());

            // 1 - The operation is the correct type
            assert_eq!(expr.as_datatype(), DataType::Integer(0));

            // 2 - The operator is the correct binary operator
            assert_eq!(Binary::from(expr.clone()).operator, Operation::Mul);
            
            // 3 - Test the operands
            test_operand_value_int(operands.0, 1);
            test_operand_value_int(operands.1, 2);
        }
        
    }
    pub fn binary_operands(bin: Binary) -> (Expression, Expression) {
        return (bin.get_left(), bin.get_right())
    }
    pub fn test_operand_value_int(expr: Expression, value: i32) {
        match expr {
            Expression::Literal(literal) => {
                match literal {
                    DataType::Integer(int) => assert_eq!(int, value),
                    _ => panic!()
                }
            },
            _ => panic!()
        }
    }
}

