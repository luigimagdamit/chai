#[allow(unused)]
pub fn llvm_main_start() -> String {
    let codegen = "define i32 @main() {\nentry:\n";
    String::from(codegen)
}#[allow(unused)]
pub fn llvm_main_close() -> String {
    let codegen = "\nret i32 0\n}";

    String::from(codegen)
}
pub fn llvm_print_define() -> String {
    let codegen = "declare i32 @printf(i8*, ...)";
    println!("{}", codegen);
    String::from(codegen)
}
pub fn llvm_print_i32_define() -> String {
    let codegen =

"\ndefine void @print_i32(i32 %value) {
    %fmt_ptr = getelementptr [4 x i8], [4 x i8]* @fmt, i32 0, i32 0
    call i32 (i8*, ...) @printf(i8* %fmt_ptr, i32 %value)
    ret void
}";
    println!("{}", codegen);
    return String::from(codegen)
}
#[allow(unused)]
pub fn llvm_call_print(index: u32, data_type: &str) {
    let codegen = format!(
        "    %result = call {} @{}()
    call void @print_{}({} %result)
", data_type, index, data_type, data_type);
    println!("{}", codegen);
}
pub fn llvm_call_print_local(index: u32, data_type: &str) -> String {
    let codegen = format!(
        "call void @print_{}({} %{})
\n", data_type, data_type, index);
    println!("{}", codegen);
    codegen
}
pub fn llvm_fmt_string_int() -> String{
    let codegen = format!("@fmt = private constant [4 x i8] c\"%d\\0A\\00\"");
    println!("{}", codegen);
    codegen
}

pub fn llvm_print_bool_declare() -> String {
    let codegen = r#"define void @print_i1(i1 %b) {
entry:
    ; Format string to print "true" or "false"
    %true_str = alloca [6 x i8], align 1
    %false_str = alloca [7 x i8], align 1

    ; Store the strings "true" and "false" in memory
    store [6 x i8] c"true\0A\00", [6 x i8]* %true_str, align 1
    store [7 x i8] c"false\0A\00", [7 x i8]* %false_str, align 1

    ; Compare the boolean value (%b) to true (1)
    %is_true = icmp eq i1 %b, true

    ; If %b is true, print "true", otherwise print "false"
    br i1 %is_true, label %print_true, label %print_false

print_true:
    ; Call printf with "true" string
    %true_ptr = getelementptr inbounds [6 x i8], [6 x i8]* %true_str, i32 0, i32 0
    call i32 @printf(i8* %true_ptr)
    br label %done

print_false:
    ; Call printf with "false" string
    %false_ptr = getelementptr inbounds [7 x i8], [7 x i8]* %false_str, i32 0, i32 0
    call i32 @printf(i8* %false_ptr)
    br label %done

done:
    ret void
}"#;

    println!("{}", codegen);
    String::from(codegen)
}