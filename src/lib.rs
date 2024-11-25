mod token;    // Import the Token and TokenType module
mod scanner;  // Import the Scanner module






#[cfg(test)]
mod tests {
    use crate::{scanner::Scanner, token::TokenType};

    #[test]
    fn test_scanner_eof() {
        let test_str = "\0";
        let s = &mut Scanner::init_scanner(test_str);
        assert_eq!(s.is_at_end(), true);
    }
    #[test]
    fn test_scanner_not_eof() {
        let test_str = "1\0";
        let s = &mut Scanner::init_scanner(&test_str);
        assert_eq!(s.is_at_end(), false);
    }
    #[test]
    fn test_scanner_not_multiple_eof() {
        let test_str = "123alkj[]!-=+\0";
        let s = &mut Scanner::init_scanner(&test_str);
        assert_eq!(s.is_at_end(), false);
    }
    #[test]
    fn test_terminated_string() {
        let test_str = "\"abcd\"\0";
        let s = &mut Scanner::init_scanner(&test_str);
        assert_eq!(s.advance().unwrap(), '\"');
        assert_eq!(s.advance().unwrap(), 'a');
        assert_eq!(s.advance().unwrap(), 'b');
        assert_eq!(s.advance().unwrap(), 'c');
        assert_eq!(s.advance().unwrap(), 'd');
        assert_eq!(s.advance().unwrap(), '\"');
        assert_eq!(s.advance().unwrap(), '\0');
        assert_eq!(s.is_at_end(), true);
    }
    #[test]
    fn test_unterminated_string() {
        let test_str = "\"abcd\0";
        let s = &mut Scanner::init_scanner(&test_str);

        let test_token = s.scan_token();

        // since the EOF is at index 5
        assert_eq!(test_token.token_type, TokenType::Error((0, 5)));
        assert_eq!(test_token.start, "Unterminated String");
        assert_eq!(s.is_at_end(), true);
    }
    #[test]
    fn test_terminals() {
        let test_str = "!!=-+><>=<=(){}\0";
        let s = &mut Scanner::init_scanner(&test_str);

        assert_eq!(s.scan_token().token_type, TokenType::Bang);
        assert_eq!(s.scan_token().token_type, TokenType::BangEqual);

        assert_eq!(s.scan_token().token_type, TokenType::Minus);
        assert_eq!(s.scan_token().token_type, TokenType::Plus);
        assert_eq!(s.scan_token().token_type, TokenType::Greater);
        assert_eq!(s.scan_token().token_type, TokenType::Less);
        assert_eq!(s.scan_token().token_type, TokenType::GreaterEqual);
        assert_eq!(s.scan_token().token_type, TokenType::LessEqual);
        assert_eq!(s.scan_token().token_type, TokenType::LeftParen);
        assert_eq!(s.scan_token().token_type, TokenType::RightParen);
        assert_eq!(s.scan_token().token_type, TokenType::LeftBrace);
        assert_eq!(s.scan_token().token_type, TokenType::RightBrace);
        assert_eq!(s.scan_token().token_type, TokenType::EOF);
        assert_eq!(s.is_at_end(), true);
    }
}
