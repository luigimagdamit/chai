pub mod test_jit{
    use serial_test::serial;
    use crate::parser::parser::Parser;
    use crate::jit::compile_test::jit_compile;
    use std::io::{self, Write};
    #[test]
    #[serial]
    fn test_print_int() {
        let test_file = "./examples/add.chai";

        let contents = std::fs::read_to_string(test_file).unwrap();
        let parser = &mut Parser::init_parser(&contents);
        parser.compile();

        let result = jit_compile(&contents).unwrap();
        assert_eq!(result, "420\n");
    }   
    #[test]
    #[serial]
    fn test_print_bool() {
        let test_file = "./examples/bool.chai";

        let contents = std::fs::read_to_string(test_file).unwrap();
        let parser = &mut Parser::init_parser(&contents);
        parser.compile();

        let result = jit_compile(&contents).unwrap();
        assert_eq!(result, "true\nfalse\ntrue\ntrue\nfalse\nfalse\ntrue\nfalse\ntrue\nfalse\n");
    }
    #[test]
    #[serial]
    fn test_fibonacci() {
        let test_file = "./examples/fib.chai";

        let contents = std::fs::read_to_string(test_file).unwrap();
        let parser = &mut Parser::init_parser(&contents);
        parser.compile();

        let result = jit_compile(&contents).unwrap();
        assert_eq!(result, "0\n1\n1\n2\n3\n5\n8\n13\n21\n34\n");
    }   
}