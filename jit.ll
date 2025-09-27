
define i32 @main() {
entry:

		%0 = add i1 0, 1

	;var x : bool = true;
	%x = alloca i1
	store i1 1, i1* %x
	%x_0 = load i1, i1* %x ; loading existing variable
		%4 = add i1 0, 1
	%5 = icmp eq i1 %x_0, 1

	;depth: 0
	br i1 %5, label %then0, label %else0
	then0:
		%7 = add i1 0, 0
	store i1 0, i1* %x
	 ; set symbol (symbol.rs)


	;; print(555);;
	call void @print_i32(i32 555); signature from PrintVisitor

	br label %end0
	
else0:
		%8 = add i1 0, 1
	store i1 1, i1* %x
	 ; set symbol (symbol.rs)


	;; print(666);;
	call void @print_i32(i32 666); signature from PrintVisitor

	br label %end0
	
end0:

	ret i32 0 ; llvm_main_close
}


	;String Constants


@fmt = private constant [4 x i8] c"%d\0A\00"
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg)
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