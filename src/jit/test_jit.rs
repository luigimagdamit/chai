pub mod test_jit{
    use serial_test::serial;
    use crate::parser::parser::Parser;
    use crate::jit::compile_test::jit_compile;
    use crate::codegen::backend_config::{init_backend_config_for_test, IRBackend};
    use std::io::{self, Write};
    #[test]
    #[serial]
    fn test_print_int() {
        init_backend_config_for_test(IRBackend::LLVM);
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
        init_backend_config_for_test(IRBackend::LLVM);
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
        init_backend_config_for_test(IRBackend::LLVM);
        let test_file = "./examples/fib.chai";

        let contents = std::fs::read_to_string(test_file).unwrap();
        let parser = &mut Parser::init_parser(&contents);
        parser.compile();

        let result = jit_compile(&contents).unwrap();
        assert_eq!(result, "0\n1\n1\n2\n3\n5\n8\n13\n21\n34\n");
    }

    #[test]
    #[serial]
    fn test_array_basic() {
        init_backend_config_for_test(IRBackend::LLVM);
        let contents = r#"
fn main() int {
    var numbers: [int] = [1, 2, 3];
    var first = numbers[0];
    print(first);
    var second = numbers[1];
    print(second);
    var third = numbers[2];
    print(third);
}
"#;

        let parser = &mut Parser::init_parser(&contents);
        parser.compile();

        let result = jit_compile(&contents).unwrap();
        assert_eq!(result, "1\n2\n3\n");
    }

    #[test]
    #[serial]
    fn test_array_single_element() {
        init_backend_config_for_test(IRBackend::LLVM);
        let contents = r#"
fn main() int {
    var single: [int] = [42];
    var value = single[0];
    print(value);
}
"#;

        let parser = &mut Parser::init_parser(&contents);
        parser.compile();

        let result = jit_compile(&contents).unwrap();
        assert_eq!(result, "42\n");
    }

    #[test]
    #[serial]
    fn test_array_large() {
        init_backend_config_for_test(IRBackend::LLVM);
        let contents = r#"
fn main() int {
    var nums: [int] = [10, 20, 30, 40, 50];
    var first = nums[0];
    print(first);
    var middle = nums[2];
    print(middle);
    var last = nums[4];
    print(last);
}
"#;

        let parser = &mut Parser::init_parser(&contents);
        parser.compile();

        let result = jit_compile(&contents).unwrap();
        assert_eq!(result, "10\n30\n50\n");
    }

    #[test]
    #[serial]
    fn test_array_boolean() {
        init_backend_config_for_test(IRBackend::LLVM);
        let contents = r#"
fn main() int {
    var flags: [bool] = [true, false, true];
    var first = flags[0];
    print(first);
    var second = flags[1];
    print(second);
    var third = flags[2];
    print(third);
}
"#;

        let parser = &mut Parser::init_parser(&contents);
        parser.compile();

        let result = jit_compile(&contents).unwrap();
        assert_eq!(result, "true\nfalse\ntrue\n");
    }

    #[test]
    #[serial]
    fn test_array_mixed_values() {
        init_backend_config_for_test(IRBackend::LLVM);
        let contents = r#"
fn main() int {
    var data: [int] = [0, 100, 999, 1];
    var zero = data[0];
    print(zero);
    var hundred = data[1];
    print(hundred);
    var big = data[2];
    print(big);
    var one = data[3];
    print(one);
}
"#;

        let parser = &mut Parser::init_parser(&contents);
        parser.compile();

        let result = jit_compile(&contents).unwrap();
        assert_eq!(result, "0\n100\n999\n1\n");
    }

    #[test]
    #[serial]
    fn test_array_type_inference() {
        init_backend_config_for_test(IRBackend::LLVM);
        let contents = r#"
fn main() int {
    var inferred = [7, 8, 9];
    var first = inferred[0];
    print(first);
    var second = inferred[1];
    print(second);
    var third = inferred[2];
    print(third);
}
"#;

        let parser = &mut Parser::init_parser(&contents);
        parser.compile();

        let result = jit_compile(&contents).unwrap();
        assert_eq!(result, "7\n8\n9\n");
    }

    #[test]
    #[serial]
    fn test_array_sequential_access() {
        init_backend_config_for_test(IRBackend::LLVM);
        let contents = r#"
fn main() int {
    var sequence: [int] = [11, 22, 33, 44, 55, 66];
    var a = sequence[0];
    var b = sequence[1];
    var c = sequence[2];
    var d = sequence[3];
    var e = sequence[4];
    var f = sequence[5];
    print(a);
    print(b);
    print(c);
    print(d);
    print(e);
    print(f);
}
"#;

        let parser = &mut Parser::init_parser(&contents);
        parser.compile();

        let result = jit_compile(&contents).unwrap();
        assert_eq!(result, "11\n22\n33\n44\n55\n66\n");
    }

    #[test]
    #[serial]
    fn test_array_negative_values() {
        init_backend_config_for_test(IRBackend::LLVM);
        let contents = r#"
fn main() int {
    var negatives: [int] = [-1, -10, -100];
    var first = negatives[0];
    print(first);
    var second = negatives[1];
    print(second);
    var third = negatives[2];
    print(third);
}
"#;

        let parser = &mut Parser::init_parser(&contents);
        parser.compile();

        let result = jit_compile(&contents).unwrap();
        assert_eq!(result, "-1\n-10\n-100\n");
    }

    #[test]
    #[serial]
    fn test_array_strings() {
        init_backend_config_for_test(IRBackend::LLVM);
        let contents = r#"
fn main() int {
    var words: [str] = ["hello", "world", "test"];
    var first = words[0];
    print(first);
    var second = words[1];
    print(second);
    var third = words[2];
    print(third);
}
"#;

        let parser = &mut Parser::init_parser(&contents);
        parser.compile();

        let result = jit_compile(&contents).unwrap();
        assert_eq!(result, "hello\nworld\ntest\n");
    }

    #[test]
    #[serial]
    fn test_array_mixed_string_operations() {
        init_backend_config_for_test(IRBackend::LLVM);
        let contents = r#"
fn main() int {
    var greetings: [str] = ["hi", "bye"];
    var msg = greetings[0];
    print(msg);
    var farewell = greetings[1];
    print(farewell);
}
"#;

        let parser = &mut Parser::init_parser(&contents);
        parser.compile();

        let result = jit_compile(&contents).unwrap();
        assert_eq!(result, "hi\nbye\n");
    }
}