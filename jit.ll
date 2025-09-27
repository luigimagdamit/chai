
define i32 @main() {
entry:

		%0 = alloca [3 x i1], align 16
		%1 = getelementptr inbounds [3 x i1], [3 x i1]* %0, i64 0, i64 0
		store i1 1, i1* %1
		%2 = getelementptr inbounds [3 x i1], [3 x i1]* %0, i64 0, i64 1
		store i1 0, i1* %2
		%3 = getelementptr inbounds [3 x i1], [3 x i1]* %0, i64 0, i64 2
		store i1 1, i1* %3

	;var flags : array[3] = array[3]:arr_0;
		%flags = alloca [3 x i1], align 16
		%flags_src = bitcast [3 x i1]* %0 to i8*
		%flags_dst = bitcast [3 x i1]* %flags to i8*
		call void @llvm.memcpy.p0i8.p0i8.i64(i8* %flags_dst, i8* %flags_src, i64 3, i1 false)
	%flags_0 = getelementptr inbounds [3 x i32], [3 x i32]* %flags, i64 0, i64 0 ; getting array pointer
		%5 = getelementptr inbounds [3 x i1], [3 x i1]* %flags, i64 0, i64 0
		%6 = load i1, i1* %5

	;var first = %6 (from temp register)
		%first = alloca i1, align 4
		store i1 %6, i1* %first
	%first_0 = load i1, i1* %first ; loading existing variable

	;; print(first);;
	call void @print_i1(i1 %first_0); signature from PrintVisitor

	%flags_1 = getelementptr inbounds [3 x i32], [3 x i32]* %flags, i64 0, i64 0 ; getting array pointer
		%8 = getelementptr inbounds [3 x i1], [3 x i1]* %flags, i64 0, i64 1
		%9 = load i1, i1* %8

	;var second = %9 (from temp register)
		%second = alloca i1, align 4
		store i1 %9, i1* %second
	%second_0 = load i1, i1* %second ; loading existing variable

	;; print(second);;
	call void @print_i1(i1 %second_0); signature from PrintVisitor

	%flags_2 = getelementptr inbounds [3 x i32], [3 x i32]* %flags, i64 0, i64 0 ; getting array pointer
		%11 = getelementptr inbounds [3 x i1], [3 x i1]* %flags, i64 0, i64 2
		%12 = load i1, i1* %11

	;var third = %12 (from temp register)
		%third = alloca i1, align 4
		store i1 %12, i1* %third
	%third_0 = load i1, i1* %third ; loading existing variable

	;; print(third);;
	call void @print_i1(i1 %third_0); signature from PrintVisitor


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