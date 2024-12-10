
define i32 @main() {
entry:

	%0 = mul i32 1, 3
	%1 = add i32 4, 2
	%2 = mul i32 %0, %1
	%3 = add i32 %2, 0				; expr_pop
	call void @print_i32(i32 %3)

	%5 = mul i32 1, 3
	%6 = add i32 4, 2
	%7 = mul i32 %5, %6
	%8 = add i32 %7, 0				; expr_pop
	call void @print_i32(i32 %8)

	%10 = add i32 1, 0				; expr_pop
	call void @print_i32(i32 %10)

	%12 = add i32 2, 0				; expr_pop
	call void @print_i32(i32 %12)

	%14 = add i1 1, 0				; expr_pop
	call void @print_i1(i1 %14)

	%16 = icmp eq i1 1, 0
	%17 = add i1 %16, 0				; expr_pop
	call void @print_i1(i1 %17)


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