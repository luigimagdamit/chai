
define i32 @main() {
entry:

	%a = alloca i32
	%0 = add i32 0, 0				; expr_pop
	store i32 %0, i32* %a			; int variable assignment (variable.rs)

	%b = alloca i32
	%1 = add i32 1, 0				; expr_pop
	store i32 %1, i32* %b			; int variable assignment (variable.rs)

	%i = alloca i32
	%2 = add i32 0, 0				; expr_pop
	store i32 %2, i32* %i			; int variable assignment (variable.rs)

	%a_0 = load i32, i32* %a 			 ; LlvmLoad load_i32
	%3 = add i32 %a_0, 0				; expr_pop
	call void @print_i32(i32 %3)

	br label %cond0

cond0:
	%i_0 = load i32, i32* %i 			 ; LlvmLoad load_i32
	%5 = icmp ult i32 %i_0, 10
;depth: 0
	%6 = add i1 %5, 0				; expr_pop
	br i1 %6, label %body0, label %exit0

body0:
	%tmp = alloca i32
	%b_0 = load i32, i32* %b 			 ; LlvmLoad load_i32
	%7 = add i32 %b_0, 0				; expr_pop
	store i32 %7, i32* %tmp			; int variable assignment (variable.rs)

	%a_1 = load i32, i32* %a 			 ; LlvmLoad load_i32
	%b_1 = load i32, i32* %b 			 ; LlvmLoad load_i32
	%8 = add i32 %a_1, %b_1
	%9 = add i32 %8, 0				; expr_pop
	store i32 %8, i32* %b		 ; set symbol (symbol.rs)

	%tmp_0 = load i32, i32* %tmp 			 ; LlvmLoad load_i32
	%10 = add i32 %tmp_0, 0				; expr_pop
	store i32 %tmp_0, i32* %a		 ; set symbol (symbol.rs)

	%tmp_1 = load i32, i32* %tmp 			 ; LlvmLoad load_i32
	%11 = add i32 %tmp_1, 0				; expr_pop
	call void @print_i32(i32 %11)

	%i_1 = load i32, i32* %i 			 ; LlvmLoad load_i32
	%13 = add i32 %i_1, 1
	%14 = add i32 %13, 0				; expr_pop
	store i32 %13, i32* %i		 ; set symbol (symbol.rs)

	br label %cond0

exit0:

	ret i32 0 ; llvm_main_close
}

;String Constants


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