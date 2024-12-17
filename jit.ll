
define i32 @main() {
entry:

	%a = alloca i32
	%0 = add i32 123, 0				; expr_pop
	store i32 %0, i32* %a			; int variable assignment (variable.rs)

	%b = alloca i32
	%1 = add i32 69, 0				; expr_pop
	store i32 %1, i32* %b			; int variable assignment (variable.rs)

	%2 = mul i32 21, 10
	%3 = mul i32 21, 10
	%4 = add i32 %2, %3
;	ast mode
;	; ((21 * 10) + (21 * 10));
	call void @print_i32(i32 %4)

	%a_0 = load i32, i32* %a 			 ; LlvmLoad load_i32
;	ast mode
;	; 123;
	%5 = add i32 123, 0
	call void @print_i32(i32 %5)

	%b_0 = load i32, i32* %b 			 ; LlvmLoad load_i32
;	ast mode
;	; 69;
	%6 = add i32 69, 0
	call void @print_i32(i32 %6)

	%a_1 = load i32, i32* %a 			 ; LlvmLoad load_i32
	%b_1 = load i32, i32* %b 			 ; LlvmLoad load_i32
	%7 = icmp sgt i32 %a_1, %b_1
;	ast mode
;	; (123 > 69);
	call void @print_i1(i1 %7)

	%a_2 = load i32, i32* %a 			 ; LlvmLoad load_i32
	%b_2 = load i32, i32* %b 			 ; LlvmLoad load_i32
	%8 = icmp slt i32 %a_2, %b_2
;	ast mode
;	; (123 < 69);
	call void @print_i1(i1 %8)

	%a_3 = load i32, i32* %a 			 ; LlvmLoad load_i32
	%b_3 = load i32, i32* %b 			 ; LlvmLoad load_i32
	%9 = icmp eq i32 %a_3, %b_3
;	ast mode
;	; (123 == 69);
	call void @print_i1(i1 %9)

	%a_4 = load i32, i32* %a 			 ; LlvmLoad load_i32
	%b_4 = load i32, i32* %b 			 ; LlvmLoad load_i32
	%10 = add i32 %b_4, 54
	%11 = icmp eq i32 %a_4, %10
;	ast mode
;	; (123 == (69 + 54));
	call void @print_i1(i1 %11)

	%12 = icmp slt i32 1, 2
;	ast mode
;	; (1 < 2);
	call void @print_i1(i1 %12)

	%13 = icmp sgt i32 3, 1
;	ast mode
;	; (3 > 1);
	call void @print_i1(i1 %13)

	%14 = icmp eq i32 2, 3
;	ast mode
;	; (2 == 3);
	call void @print_i1(i1 %14)

;	ast mode
;	; bool <true>;
	%15 = add i1 1, 0
	call void @print_i1(i1 %15)

;	ast mode
;	; bool <false>;
	%16 = add i1 0, 0
	call void @print_i1(i1 %16)


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