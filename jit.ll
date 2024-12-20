
define i32 @main() {
entry:


	;	; pushing a new string on the stack ..."hello world!"
	%0 = getelementptr inbounds [14 x i8], [14 x i8]* @str0, i32 0, i32 0 ; llvm_retrieve_static_string ; place() in impl StringConstant

	;var greeting : str = "hello world!";
	%greeting = alloca i8*
	store i8* %0, i8** %greeting

	;	; pushing a new string on the stack ..."this is chai"
	%2 = getelementptr inbounds [14 x i8], [14 x i8]* @str1, i32 0, i32 0 ; llvm_retrieve_static_string ; place() in impl StringConstant

	;var intro : str = "this is chai";
	%intro = alloca i8*
	store i8* %2, i8** %intro
	%greeting_0 = load i8*, i8** %greeting ; loading existing variable

	;; print(greeting);;
	call i32 (i8*, ...) @printf(i8* %greeting_0)
	%greeting_1 = load i8*, i8** %greeting ; loading existing variable

	;; print(greeting);;
	call i32 (i8*, ...) @printf(i8* %greeting_1)
	%intro_0 = load i8*, i8** %intro ; loading existing variable

	;; print(intro);;
	call i32 (i8*, ...) @printf(i8* %intro_0)
	%intro_1 = load i8*, i8** %intro ; loading existing variable

	;; print(intro);;
	call i32 (i8*, ...) @printf(i8* %intro_1)
	%greeting_2 = load i8*, i8** %greeting ; loading existing variable

	;; print(greeting);;
	call i32 (i8*, ...) @printf(i8* %greeting_2)

	;var a : int = 123
	%a = alloca i32
	store i32 123, i32* %a

	;var b : int = 69
	%b = alloca i32
	store i32 69, i32* %b
	%a_0 = load i32, i32* %a ; loading existing variable

	;; print(a);;
	call void @print_i32(i32 %a_0); signature from PrintVisitor

	%b_0 = load i32, i32* %b ; loading existing variable

	;; print(b);;
	call void @print_i32(i32 %b_0); signature from PrintVisitor

	%a_1 = load i32, i32* %a ; loading existing variable
	%b_1 = load i32, i32* %b ; loading existing variable
	%13 = icmp sgt i32 %a_1, %b_1

	;; print((a > b));;
	call void @print_i1(i1 %13); signature from PrintVisitor

	%a_2 = load i32, i32* %a ; loading existing variable
	%b_2 = load i32, i32* %b ; loading existing variable
	%14 = icmp slt i32 %a_2, %b_2

	;; print((a < b));;
	call void @print_i1(i1 %14); signature from PrintVisitor

	%a_3 = load i32, i32* %a ; loading existing variable
	%b_3 = load i32, i32* %b ; loading existing variable
	%15 = icmp eq i32 %a_3, %b_3

	;; print((a == b));;
	call void @print_i1(i1 %15); signature from PrintVisitor

	%a_4 = load i32, i32* %a ; loading existing variable
	%b_4 = load i32, i32* %b ; loading existing variable
	%16 = add i32 %b_4, 54
	%17 = icmp eq i32 %a_4, %16

	;; print((a == (b + 54)));;
	call void @print_i1(i1 %17); signature from PrintVisitor

	%18 = getelementptr inbounds [13 x i8], [13 x i8]* @str0, i32 0, i32 0 ; llvm_retrieve_static_string ; place() in impl StringConstant
	call i32 (i8*, ...) @printf(i8* %18)
	%20 = getelementptr inbounds [13 x i8], [13 x i8]* @str1, i32 0, i32 0 ; llvm_retrieve_static_string ; place() in impl StringConstant
	call i32 (i8*, ...) @printf(i8* %20)

	;	; pushing a new string on the stack ..."here is me declaring a variable"
	%22 = getelementptr inbounds [33 x i8], [33 x i8]* @str2, i32 0, i32 0 ; llvm_retrieve_static_string ; place() in impl StringConstant
	call i32 (i8*, ...) @printf(i8* %22)

	;	; pushing a new string on the stack ..."this is the variable"
	%24 = getelementptr inbounds [22 x i8], [22 x i8]* @str3, i32 0, i32 0 ; llvm_retrieve_static_string ; place() in impl StringConstant
	call i32 (i8*, ...) @printf(i8* %24)

	;var i : int = 24
	%i = alloca i32
	store i32 24, i32* %i
	%i_0 = load i32, i32* %i ; loading existing variable

	;; print(i);;
	call void @print_i32(i32 %i_0); signature from PrintVisitor

	%flag = alloca i1
	
	%29 = icmp slt i32 1, 2

	;; print((1 < 2));;
	call void @print_i1(i1 %29); signature from PrintVisitor

	%30 = icmp sgt i32 3, 1

	;; print((3 > 1));;
	call void @print_i1(i1 %30); signature from PrintVisitor

	%31 = icmp eq i32 2, 3

	;; print((2 == 3));;
	call void @print_i1(i1 %31); signature from PrintVisitor


	;; print(bool <true>);;
	call void @print_i1(i1 1); signature from PrintVisitor


	;; print(bool <false>);;
	call void @print_i1(i1 0); signature from PrintVisitor


	;	; pushing a new string on the stack ..."ok shutting down now"
	%32 = getelementptr inbounds [22 x i8], [22 x i8]* @str4, i32 0, i32 0 ; llvm_retrieve_static_string ; place() in impl StringConstant

	;var seppuku : str = "ok shutting down now";
	%seppuku = alloca i8*
	store i8* %32, i8** %seppuku
	%seppuku_0 = load i8*, i8** %seppuku ; loading existing variable

	;; print(seppuku);;
	call i32 (i8*, ...) @printf(i8* %seppuku_0)

	ret i32 0 ; llvm_main_close
}


	;String Constants
@str2 = private unnamed_addr constant [33 x i8] c"here is me declaring a variable\0A\00", align 1
@str1 = private unnamed_addr constant [14 x i8] c"this is chai\0A\00", align 1
@str4 = private unnamed_addr constant [22 x i8] c"ok shutting down now\0A\00", align 1
@str3 = private unnamed_addr constant [22 x i8] c"this is the variable\0A\00", align 1
@str0 = private unnamed_addr constant [14 x i8] c"hello world!\0A\00", align 1


@fmt = private constant [4 x i8] c"%d\0A\00"
declare i32 @printf(i8*, ...)
define void @print_i32(i32 %value) {
    %fmt_ptr = getelementptr [4 x i8], [4 x i8]* @fmt, i32 0, i32 0
    call i32 (i8*, ...) @printf(i8* %fmt_ptr, i32 %value)
    ret void
}
define void @print_i1(i1 %b) {
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
}