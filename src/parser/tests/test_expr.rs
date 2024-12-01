mod tests {
    #![allow(unused_imports)]
    use core::panic;

    use crate::common::common::PARSE_CONSTANT_FOLD;
    use crate::parser::expr::DataType;
    use crate::parser::parse_fn::parse_precedence;
    use crate::scanner::token::TokenType;
    use crate::parser::parser::Parser;
    use crate::parser::precedence::Precedence;
    #[test] 
    fn test_parse_one_plus_two() {
        let parser = &mut Parser::init_parser("1+2\0");
        parser.advance();
        parse_precedence(parser, Precedence::PrecAssignment);

        if let Some(c) = &parser.constant_stack.pop().unwrap() {
            match c.data_type {
                DataType::Integer(value) => assert_eq!(value, 3),
                _ => {}
            }
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
            if PARSE_CONSTANT_FOLD {
                assert_eq!(c.left, test_left);
                assert_eq!(c.right, test_right);
            } else {
                match c.data_type {
                    DataType::Integer(value) => assert_eq!(value, 420),
                    _ => {}
                }
            }
            
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
            if PARSE_CONSTANT_FOLD {
                assert_eq!(c.left, test_left);
                assert_eq!(c.right, test_right);
            } else {
                match c.data_type {
                    DataType::Integer(value) => assert_eq!(value, 18),
                    _ => {}
                }
            }
            
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
            if PARSE_CONSTANT_FOLD {
                assert_eq!(c.left, test_left);
                assert_eq!(c.right, test_right);
            } else {
                match c.data_type {
                    DataType::Integer(value) => assert_eq!(value, 66),
                    _ => {}
                }
            }
            
        } else {
            panic!();
        }
    }
   


}
mod boolean_equality_tests {
    #![allow(unused_imports)]
    use core::panic;

    use crate::common::common::PARSE_CONSTANT_FOLD;
    use crate::parser::expr::DataType;
    use crate::parser::parse_fn::parse_precedence;

    use crate::parser::parser::Parser;
    use crate::parser::precedence::Precedence;
    #[test]
    fn test_parse_bool_true() {
        let parser = &mut Parser::init_parser("true\0");
        parser.advance();
        parse_precedence(parser, Precedence::PrecAssignment);

        if let Some(c) = &parser.constant_stack.pop().unwrap() {
            match c.data_type {
                DataType::Boolean(value) => assert_eq!(value, true),
                _ => ()
            }
        }
    }
    #[test]
    fn test_parse_bool_false() {
        let parser = &mut Parser::init_parser("false\0");
        let test_val = false;
        parser.advance();
        parse_precedence(parser, Precedence::PrecAssignment);

        if let Some(c) = &parser.constant_stack.pop().unwrap() {
            match c.data_type {
                DataType::Boolean(value) => assert_eq!(value, test_val),
                _ => ()
            }
        }
    }
    #[test]
    fn test_parse_bool_eq_tt() {
        let parser = &mut Parser::init_parser("true == true\0");
        let test_val = true == true;
        parser.advance();
        parse_precedence(parser, Precedence::PrecAssignment);

        if let Some(c) = &parser.constant_stack.pop().unwrap() {
            match c.data_type {
                DataType::Boolean(value) => assert_eq!(value, test_val),
                _ => ()
            }
        }
    }
    #[test]
    fn test_parse_bool_eq_tf() {
        let parser = &mut Parser::init_parser("true == false\0");
        let test_val = true == false;
        parser.advance();
        parse_precedence(parser, Precedence::PrecAssignment);

        if let Some(c) = &parser.constant_stack.pop().unwrap() {
            match c.data_type {
                DataType::Boolean(value) => assert_eq!(value, test_val),
                _ => ()
            }
        }
    }
    #[test]
    fn test_parse_bool_eq_ft() {
        let parser = &mut Parser::init_parser("false == true\0");
        let test_val = false == true;
        parser.advance();
        parse_precedence(parser, Precedence::PrecAssignment);

        if let Some(c) = &parser.constant_stack.pop().unwrap() {
            match c.data_type {
                DataType::Boolean(value) => assert_eq!(value, test_val),
                _ => ()
            }
        }
    }
    #[test]
    fn test_parse_bool_eq_ff() {
        let parser = &mut Parser::init_parser("false == false\0");
        let test_val = false == false;
        parser.advance();
        parse_precedence(parser, Precedence::PrecAssignment);

        if let Some(c) = &parser.constant_stack.pop().unwrap() {
            match c.data_type {
                DataType::Boolean(value) => assert_eq!(value, test_val),
                _ => ()
            }
        }
    }

    #[test]
    fn test_parse_bool_neq_tt() {
        let parser = &mut Parser::init_parser("true != true\0");
        let test_val = true != true;
        parser.advance();
        parse_precedence(parser, Precedence::PrecAssignment);

        if let Some(c) = &parser.constant_stack.pop().unwrap() {
            match c.data_type {
                DataType::Boolean(value) => assert_eq!(value, test_val),
                _ => ()
            }
        }
    }
    #[test]
    fn test_parse_bool_neq_tf() {
        let parser = &mut Parser::init_parser("true != false\0");
        let test_val = true != false;
        parser.advance();
        parse_precedence(parser, Precedence::PrecAssignment);

        if let Some(c) = &parser.constant_stack.pop().unwrap() {
            match c.data_type {
                DataType::Boolean(value) => assert_eq!(value, test_val),
                _ => ()
            }
        }
    }
    #[test]
    fn test_parse_bool_neq_ft() {
        let parser = &mut Parser::init_parser("false != true\0");
        let test_val = false != true;
        parser.advance();
        parse_precedence(parser, Precedence::PrecAssignment);

        if let Some(c) = &parser.constant_stack.pop().unwrap() {
            match c.data_type {
                DataType::Boolean(value) => assert_eq!(value, test_val),
                _ => ()
            }
        }
    }
    #[test]
    fn test_parse_bool_neq_ff() {
        let parser = &mut Parser::init_parser("false != false\0");
        let test_val = false != false;
        parser.advance();
        parse_precedence(parser, Precedence::PrecAssignment);

        if let Some(c) = &parser.constant_stack.pop().unwrap() {
            match c.data_type {
                DataType::Boolean(value) => assert_eq!(value, test_val),
                _ => ()
            }
        }
    }
}

mod value_comparison_tests {
    #![allow(unused_imports)]
    use core::panic;

    use crate::common::common::PARSE_CONSTANT_FOLD;
    use crate::parser::expr::DataType;
    use crate::parser::parse_fn::parse_precedence;

    use crate::parser::parser::Parser;
    use crate::parser::precedence::Precedence;
    #[test]
    fn test_parse_numbers_eq_true() {
        let parser = &mut Parser::init_parser("1 == 1\0");
        let test_val = 1 == 1;
        parser.advance();
        parse_precedence(parser, Precedence::PrecAssignment);

        if let Some(c) = &parser.constant_stack.pop().unwrap() {
            match c.data_type {
                DataType::Boolean(value) => assert_eq!(value, test_val),
                _ => ()
            }
        }
    }

    #[test]
    fn test_parse_numbers_eq_false() {
        let parser = &mut Parser::init_parser("1 == 2\0");
        let test_val = 1 == 2;
        parser.advance();
        parse_precedence(parser, Precedence::PrecAssignment);

        if let Some(c) = &parser.constant_stack.pop().unwrap() {
            match c.data_type {
                DataType::Boolean(value) => assert_eq!(value, test_val),
                _ => ()
            }
        }
    }

    #[test]
    fn test_parse_numbers_neq_true() {
        let parser = &mut Parser::init_parser("1 != 2\0");
        let test_val = 1 != 2;
        parser.advance();
        parse_precedence(parser, Precedence::PrecAssignment);

        if let Some(c) = &parser.constant_stack.pop().unwrap() {
            match c.data_type {
                DataType::Boolean(value) => assert_eq!(value, test_val),
                _ => ()
            }
        }
    }
    #[test]
    fn test_parse_numbers_neq_false() {
        let parser = &mut Parser::init_parser("2 != 2\0");
        let test_val = 2 != 2;
        parser.advance();
        parse_precedence(parser, Precedence::PrecAssignment);

        if let Some(c) = &parser.constant_stack.pop().unwrap() {
            match c.data_type {
                DataType::Boolean(value) => assert_eq!(value, test_val),
                _ => ()
            }
        }
    }
    #[test]
    fn test_parse_numbers_gt_true() {
        let parser = &mut Parser::init_parser("3 > 2\0");
        let test_val = 3 > 2;
        parser.advance();
        parse_precedence(parser, Precedence::PrecAssignment);

        if let Some(c) = &parser.constant_stack.pop().unwrap() {
            match c.data_type {
                DataType::Boolean(value) => assert_eq!(value, test_val),
                _ => ()
            }
        }
    }
    #[test]
    fn test_parse_numbers_gt_false() {
        let parser = &mut Parser::init_parser("1 > 3\0");
        let test_val = 1 > 3;
        parser.advance();
        parse_precedence(parser, Precedence::PrecAssignment);

        if let Some(c) = &parser.constant_stack.pop().unwrap() {
            match c.data_type {
                DataType::Boolean(value) => assert_eq!(value, test_val),
                _ => ()
            }
        }
    }
    #[test]
    fn test_parse_numbers_lt_true() {
        let parser = &mut Parser::init_parser("4 < 5\0");
        let test_val = 4 < 5;
        parser.advance();
        parse_precedence(parser, Precedence::PrecAssignment);

        if let Some(c) = &parser.constant_stack.pop().unwrap() {
            match c.data_type {
                DataType::Boolean(value) => assert_eq!(value, test_val),
                _ => ()
            }
        }
    }
    #[test]
    fn test_parse_numbers_lt_false() {
        let parser = &mut Parser::init_parser("6 < 5\0");
        let test_val = 6 < 5;
        parser.advance();
        parse_precedence(parser, Precedence::PrecAssignment);

        if let Some(c) = &parser.constant_stack.pop().unwrap() {
            match c.data_type {
                DataType::Boolean(value) => assert_eq!(value, test_val),
                _ => ()
            }
        }
    }
}