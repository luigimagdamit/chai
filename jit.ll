
define i32 @main() {
entry:


	;var a : int = 0
	%a = alloca i32
	store i32 0, i32* %a

	;var b : int = 1
	%b = alloca i32
	store i32 1, i32* %b

	;var i : int = 0
	%i = alloca i32
	store i32 0, i32* %i
	%a_0 = load i32, i32* %a ; loading existing variable

	;; print(a);;
	call void @print_i32(i32 %a_0); signature from PrintVisitor

	br label %cond3
	
cond3:
	%i_0 = load i32, i32* %i ; loading existing variable
	%3 = icmp slt i32 %i_0, 9

	;depth: 3
	br i1 %3, label %body3, label %exit3
	
body3:
	%b_0 = load i32, i32* %b ; loading existing variable

	;var tmp : int = b
	%tmp = alloca i32
	store i32 %b_0, i32* %tmp
	%a_1 = load i32, i32* %a ; loading existing variable
	%b_1 = load i32, i32* %b ; loading existing variable
	%6 = add i32 %a_1, %b_1
	store i32 %6, i32* %b
	 ; set symbol (symbol.rs)

	%tmp_0 = load i32, i32* %tmp ; loading existing variable
	store i32 %tmp_0, i32* %a
	 ; set symbol (symbol.rs)

	%tmp_1 = load i32, i32* %tmp ; loading existing variable

	;; print(tmp);;
	call void @print_i32(i32 %tmp_1); signature from PrintVisitor

	%i_1 = load i32, i32* %i ; loading existing variable
	%7 = add i32 %i_1, 1
	store i32 %7, i32* %i
	 ; set symbol (symbol.rs)

	br label %cond3
	
exit3:

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