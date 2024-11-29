mod tests {
    use core::panic;

    use crate::parser::parse_fn::parse_precedence;
    use crate::scanner::token::TokenType;
    use crate::parser::parser::Parser;
    use crate::parser::precedence::Precedence;

    #[test]
    fn test_parser_eof() {
        let parser = &mut Parser::init_parser("\0");
        parser.advance();
        assert_eq!(parser.current.unwrap().token_type, TokenType::EOF);
    }
    #[test]
    fn test_parser_single_number() {
        let parser = &mut Parser::init_parser("123\0");
        parser.advance();
        if let Some(number_token) = parser.current {
            assert_eq!(number_token.token_type, TokenType::Number);
            assert_eq!(number_token.start, "123");
        }
        
    }
    #[test]
    fn test_parser_two_numbers() {
        let parser = &mut Parser::init_parser("123 321\0");
        parser.advance();
        if let Some(number_token) = parser.current {
            assert_eq!(number_token.token_type, TokenType::Number);
            assert_eq!(number_token.start, "123");
        }
        parser.advance();
        if let Some(number_token) = parser.current {
            assert_eq!(number_token.token_type, TokenType::Number);
            assert_eq!(number_token.start, "321");
        }
        
    }
    #[test] 
    fn test_parse_one_plus_two() {
        let parser = &mut Parser::init_parser("1+2\0");
        parser.advance();
        parse_precedence(parser, Precedence::PrecAssignment);

        if let Some(c) = &parser.constant_stack.pop().unwrap() {
            assert_eq!(c.left, "i32 3");
            assert_eq!(c.right, "3");
        } else {
            panic!();
        }
        
    }
    #[test]
    fn test_parse_multiple_operands() {
        let test_left = "i32 420";
        let test_right = "420";
        let parser = &mut Parser::init_parser("21 * 10 + 21 * 10\0");
        parser.advance();
        parse_precedence(parser, Precedence::PrecAssignment);

        if let Some(c) = &parser.constant_stack.pop().unwrap() {
            assert_eq!(c.left, test_left);
            assert_eq!(c.right, test_right);
        } else {
            panic!();
        }
        
    }
    #[test]
    fn test_parse_multiple_operand2s() {
        let test_left = "i32 18";
        let test_right = "18";
        let parser = &mut Parser::init_parser("8+4*3-2\0");
        parser.advance();
        parse_precedence(parser, Precedence::PrecAssignment);

        if let Some(c) = &parser.constant_stack.pop().unwrap() {
            assert_eq!(c.left, test_left);
            assert_eq!(c.right, test_right);
        } else {
            panic!();
        }
    }
    #[test]
    fn test_parse_grouping() {
        let test_left = "i32 66";
        let test_right = "66";
        let parser = &mut Parser::init_parser("(800 * 2) / (4 + 20)\0");
        parser.advance();
        parse_precedence(parser, Precedence::PrecAssignment);

        if let Some(c) = &parser.constant_stack.pop().unwrap() {
            assert_eq!(c.left, test_left);
            assert_eq!(c.right, test_right);
        } else {
            panic!();
        }
    }
}