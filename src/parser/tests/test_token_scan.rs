mod tests {
    #![allow(unused_imports)]
    use core::panic;
   
    use crate::common::flags::PARSE_CONSTANT_FOLD;
    use crate::parser::expression::expr::DataType;
    use crate::parser::expression::expression::parse_precedence;
    use crate::scanner::token::TokenType;
    use crate::parser::parser::Parser;
    use crate::parser::expression::precedence::Precedence;

    #[test]
    fn test_scan_eof() {
        let parser = &mut Parser::init_parser("\0");
        parser.advance();
        assert_eq!(parser.current.unwrap().token_type, TokenType::EOF);
    }
    #[test]
    fn test_scan_single_number() {
        let parser = &mut Parser::init_parser("123\0");
        parser.advance();
        if let Some(number_token) = parser.current {
            assert_eq!(number_token.token_type, TokenType::Number);
            assert_eq!(number_token.start, "123");
        }
        
    }
    #[test]
    fn test_scan_two_numbers() {
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
    fn test_scan_string() {
        let parser = &mut Parser::init_parser("\"grinch\"\0");
        parser.advance();
        if let Some(number_token) = parser.current {
            assert_eq!(number_token.token_type, TokenType::String);
            assert_eq!(number_token.start, "\"grinch\"");
        }
    }
    #[test]
    fn test_scan_true() {
        let parser = &mut Parser::init_parser("true\0");
        parser.advance();
        if let Some(number_token) = parser.current {
            assert_eq!(number_token.token_type, TokenType::True);
            assert_eq!(number_token.start, "true");
        }
    }
    #[test]
    fn test_scan_false() {
        let parser = &mut Parser::init_parser("false\0");
        parser.advance();
        if let Some(number_token) = parser.current {
            assert_eq!(number_token.token_type, TokenType::False);
            assert_eq!(number_token.start, "false");
        }
    }
}