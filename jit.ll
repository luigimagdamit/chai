
define i32 @main() {
entry:

	%a = alloca i32store i1 1, i1* %a

	%a_0 = load i1, i1* %a
;	ast mode
;	; a;
	call void @print_i1(i1 %a_0); signature from PrintVisitor

	store i1 0, i1* %a		 ; set symbol (symbol.rs)


	%a_1 = load i1, i1* %a
;	ast mode
;	; a;
	call void @print_i1(i1 %a_1); signature from PrintVisitor

	%b = alloca i32store i1 0, i1* %b

	%a_2 = load i1, i1* %a

	%b_0 = load i1, i1* %b
	%0 = icmp eq i1 %a_2, %b_0
;	ast mode
;	; (a == b);
	call void @print_i1(i1 %0); signature from PrintVisitor


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