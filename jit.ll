
define i32 @main() {
entry:


	;	; pushing a new string on the stack ..."hello world!"
	%0 = getelementptr inbounds [14 x i8], [14 x i8]* @str0, i32 0, i32 0 ; llvm_retrieve_static_string ; place() in impl StringConstant
	call i32 (i8*, ...) @printf(i8* %0)

	;	; pushing a new string on the stack ..."beignets"
	%2 = getelementptr inbounds [10 x i8], [10 x i8]* @str1, i32 0, i32 0 ; llvm_retrieve_static_string ; place() in impl StringConstant
	%breakfast = alloca i8*
	store i8* %2, i8** %breakfast

	;	; pushing a new string on the stack ..."cafe au lait"
	%4 = getelementptr inbounds [14 x i8], [14 x i8]* @str2, i32 0, i32 0 ; llvm_retrieve_static_string ; place() in impl StringConstant
	%beverage = alloca i8*
	store i8* %4, i8** %beverage
	%breakfast_0 = load i8*, i8** %breakfast ; loading existing variable

	;; print(breakfast);;
	call i32 (i8*, ...) @printf(i8* %breakfast_0)
	%beverage_0 = load i8*, i8** %beverage ; loading existing variable

	;; print(beverage);;
	call i32 (i8*, ...) @printf(i8* %beverage_0)
	%hungry = alloca i1
	store i1 1, i1* %hungry
	%snacks = alloca i32
	store i32 10, i32* %snacks
	br label %cond8
	
cond8:
	%snacks_0 = load i32, i32* %snacks ; loading existing variable
	%8 = icmp ne i32 %snacks_0, 0

	;depth: 8
	br i1 %8, label %body8, label %exit8
	
body8:
	%snacks_1 = load i32, i32* %snacks ; loading existing variable
	%10 = sub i32 %snacks_1, 1
	store i32 %10, i32* %snacks
	 ; set symbol (symbol.rs)

	%hungry_0 = load i1, i1* %hungry ; loading existing variable
	%14 = icmp eq i1 %hungry_0, 1

	;depth: 1
	br i1 %14, label %then1, label %else1
	then1:

	;	; pushing a new string on the stack ..."lets eat!"
	%16 = getelementptr inbounds [11 x i8], [11 x i8]* @str3, i32 0, i32 0 ; llvm_retrieve_static_string ; place() in impl StringConstant
	call i32 (i8*, ...) @printf(i8* %16)

	;	; pushing a new string on the stack ..."nom"
	%18 = getelementptr inbounds [5 x i8], [5 x i8]* @str4, i32 0, i32 0 ; llvm_retrieve_static_string ; place() in impl StringConstant
	call i32 (i8*, ...) @printf(i8* %18)
	store i1 0, i1* %hungry
	 ; set symbol (symbol.rs)

	br label %end1
	
else1:

	;	; pushing a new string on the stack ..."ima sleep"
	%20 = getelementptr inbounds [11 x i8], [11 x i8]* @str5, i32 0, i32 0 ; llvm_retrieve_static_string ; place() in impl StringConstant
	call i32 (i8*, ...) @printf(i8* %20)

	;	; pushing a new string on the stack ..."zzz"
	%22 = getelementptr inbounds [5 x i8], [5 x i8]* @str6, i32 0, i32 0 ; llvm_retrieve_static_string ; place() in impl StringConstant
	call i32 (i8*, ...) @printf(i8* %22)
	store i1 1, i1* %hungry
	 ; set symbol (symbol.rs)

	br label %end1
	
end1:
	br label %cond8
	
exit8:

	ret i32 0 ; llvm_main_close
}


	;String Constants
@str5 = private unnamed_addr constant [11 x i8] c"ima sleep\0A\00", align 1
@str0 = private unnamed_addr constant [14 x i8] c"hello world!\0A\00", align 1
@str2 = private unnamed_addr constant [14 x i8] c"cafe au lait\0A\00", align 1
@str1 = private unnamed_addr constant [10 x i8] c"beignets\0A\00", align 1
@str3 = private unnamed_addr constant [11 x i8] c"lets eat!\0A\00", align 1
@str4 = private unnamed_addr constant [5 x i8] c"nom\0A\00", align 1
@str6 = private unnamed_addr constant [5 x i8] c"zzz\0A\00", align 1


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