use crate::parser::expression::expr::Expression;

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
            let expr = Expression::from(ast_node.to_expression());
            let binary = Binary::from(expr.clone());
            let operands = binary_operands(binary.clone());
            assert_eq!(expr.as_datatype(), DataType::Integer(0)); // test if binary type is correct
            assert_eq!(Binary::from(expr.clone()).operator, Operation::Add);
            // Assert that left operand is 1
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

