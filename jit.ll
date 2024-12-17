
define i32 @main() {
entry:

	%0 = add i32 12, 0				; expr_pop
	%a = alloca i32
	store i32 12, i32* %a ; signature by visitor

	%1 = mul i32 10, 21
	%2 = mul i32 10, 21
	%3 = add i32 %2, %1
	%4 = add i32 10, 0				; expr_pop
	%b = alloca i32
	store i32 %3, i32* %b ; signature by visitor

	%b_0 = load i32, i32* %b
;	ast mode
;	; ;
	call void @print_i32(i32 %b_0); signature from PrintVisitor

	%5 = mul i32 10, 21
;	ast mode
;	; (10 * 21);
	call void @print_i32(i32 %5); signature from PrintVisitor

	%6 = icmp eq i1 1, 1
;	ast mode
;	; (bool <true> == bool <true>);
	call void @print_i1(i1 %6); signature from PrintVisitor

	%7 = icmp eq i1 1, 0
;	ast mode
;	; (bool <true> == bool <false>);
	call void @print_i1(i1 %7); signature from PrintVisitor

;	ast mode
;	; 21;
	call void @print_i32(i32 21); signature from PrintVisitor


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