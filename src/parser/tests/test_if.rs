mod test_if {
    use serial_test::serial;
    use crate::parser::parser::Parser;
    use crate::jit::compile_test::jit_compile;
    use crate::codegen::backend_config::{init_backend_config_for_test, IRBackend};

    #[test]
    #[serial]
    fn test_if_true_condition() {
        init_backend_config_for_test(IRBackend::LLVM);

        let source = r#"
            fn main() int {
                if (true) {
                    print(123);
                } else {
                    print(456);
                }
            }
        "#;

        let result = jit_compile(source).unwrap();
        assert_eq!(result, "123\n");
    }

    #[test]
    #[serial]
    fn test_if_false_condition() {
        init_backend_config_for_test(IRBackend::LLVM);

        let source = r#"
            fn main() int {
                if (false) {
                    print(123);
                } else {
                    print(456);
                }
            }
        "#;

        let result = jit_compile(source).unwrap();
        assert_eq!(result, "456\n");
    }

    #[test]
    #[serial]
    fn test_if_boolean_variable_true() {
        init_backend_config_for_test(IRBackend::LLVM);

        let source = r#"
            fn main() int {
                var flag: bool = true;
                if (flag == true) {
                    print(111);
                } else {
                    print(222);
                }
            }
        "#;

        let result = jit_compile(source).unwrap();
        assert_eq!(result, "111\n");
    }

    #[test]
    #[serial]
    fn test_if_boolean_variable_false() {
        init_backend_config_for_test(IRBackend::LLVM);

        let source = r#"
            fn main() int {
                var flag: bool = false;
                if (flag == true) {
                    print(111);
                } else {
                    print(222);
                }
            }
        "#;

        let result = jit_compile(source).unwrap();
        assert_eq!(result, "222\n");
    }

    #[test]
    #[serial]
    fn test_if_boolean_equality_false() {
        init_backend_config_for_test(IRBackend::LLVM);

        let source = r#"
            fn main() int {
                var hungry: bool = true;
                if (hungry == false) {
                    print(333);
                } else {
                    print(444);
                }
            }
        "#;

        let result = jit_compile(source).unwrap();
        assert_eq!(result, "444\n");
    }

    #[test]
    #[serial]
    fn test_if_with_multiple_statements() {
        init_backend_config_for_test(IRBackend::LLVM);

        let source = r#"
            fn main() int {
                if (true) {
                    print(100);
                    print(200);
                } else {
                    print(300);
                    print(400);
                }
            }
        "#;

        let result = jit_compile(source).unwrap();
        assert_eq!(result, "100\n200\n");
    }

    #[test]
    #[serial]
    fn test_if_else_with_multiple_statements() {
        init_backend_config_for_test(IRBackend::LLVM);

        let source = r#"
            fn main() int {
                if (false) {
                    print(100);
                    print(200);
                } else {
                    print(300);
                    print(400);
                }
            }
        "#;

        let result = jit_compile(source).unwrap();
        assert_eq!(result, "300\n400\n");
    }

    #[test]
    #[serial]
    fn test_if_with_variable_assignment() {
        init_backend_config_for_test(IRBackend::LLVM);

        let source = r#"
            fn main() int {
                var x: bool = true;
                if (x == true) {
                    x = false;
                    print(555);
                } else {
                    x = true;
                    print(666);
                }
            }
        "#;

        let result = jit_compile(source).unwrap();
        assert_eq!(result, "555\n");
    }

    #[test]
    #[serial]
    fn test_if_string_output() {
        init_backend_config_for_test(IRBackend::LLVM);

        let source = r#"
            fn main() int {
                if (true) {
                    print("hello");
                } else {
                    print("world");
                }
            }
        "#;

        let result = jit_compile(source).unwrap();
        assert_eq!(result, "hello\n");
    }

    #[test]
    #[serial]
    fn test_nested_if_simulation() {
        init_backend_config_for_test(IRBackend::LLVM);

        let source = r#"
            fn main() int {
                var a: bool = true;
                var b: bool = false;
                if (a == true) {
                    print(777);
                    if (b == false) {
                        print(888);
                    } else {
                        print(999);
                    }
                } else {
                    print(000);
                }
            }
        "#;

        let result = jit_compile(source).unwrap();
        assert_eq!(result, "777\n888\n");
    }
}