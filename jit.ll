
define i32 @main() {
entry:

	%a = alloca i32
	store i32 0, i32* %a ; signature by visitor

	%b = alloca i32
	store i32 1, i32* %b ; signature by visitor

	%i = alloca i32
	store i32 0, i32* %i ; signature by visitor

%a_0 = load i32, i32* %a
;	ast mode
;	; ;
	call void @print_i32(i32 %a_0); signature from PrintVisitor

	br label %cond0

cond0:
%i_0 = load i32, i32* %i
	%0 = icmp slt i32 %i_0, 9
;depth: 0
	%1 = add i32 9, 0				; expr_pop
	br i1 %0, label %body0, label %exit0

body0:
%b_0 = load i32, i32* %b
	%tmp = alloca i32
	store i32 %b_0, i32* %tmp ; signature by visitor

%a_1 = load i32, i32* %a
%b_1 = load i32, i32* %b
	%2 = add i32 %a_1, %b_1
	store i32 %2, i32* %b		 ; set symbol (symbol.rs)

%tmp_0 = load i32, i32* %tmp
	store i32 %tmp_0, i32* %a		 ; set symbol (symbol.rs)

%tmp_1 = load i32, i32* %tmp
;	ast mode
;	; ;
	call void @print_i32(i32 %tmp_1); signature from PrintVisitor

%i_1 = load i32, i32* %i
	%3 = add i32 %i_1, 1
	store i32 %3, i32* %i		 ; set symbol (symbol.rs)

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