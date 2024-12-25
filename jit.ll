
define i32 @main() {
entry:


	;; print(bool <true>);;
	call void @print_i1(i1 1); signature from PrintVisitor


	;; print(bool <false>);;
	call void @print_i1(i1 0); signature from PrintVisitor

	%0 = icmp eq i1 1, 1

	;; print((bool <true> == bool <true>));;
	call void @print_i1(i1 %0); signature from PrintVisitor

	%1 = icmp eq i1 0, 0

	;; print((bool <false> == bool <false>));;
	call void @print_i1(i1 %1); signature from PrintVisitor

	%2 = icmp eq i1 1, 0

	;; print((bool <true> == bool <false>));;
	call void @print_i1(i1 %2); signature from PrintVisitor

	%3 = icmp eq i1 0, 1

	;; print((bool <false> == bool <true>));;
	call void @print_i1(i1 %3); signature from PrintVisitor

	%4 = icmp ne i1 1, 0

	;; print((bool <true> != bool <false>));;
	call void @print_i1(i1 %4); signature from PrintVisitor

	%5 = icmp ne i1 1, 1

	;; print((bool <true> != bool <true>));;
	call void @print_i1(i1 %5); signature from PrintVisitor

	%6 = icmp ne i1 0, 1

	;; print((bool <false> != bool <true>));;
	call void @print_i1(i1 %6); signature from PrintVisitor

	%7 = icmp ne i1 0, 0

	;; print((bool <false> != bool <false>));;
	call void @print_i1(i1 %7); signature from PrintVisitor


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