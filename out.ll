declare i32 @printf(i8*, ...)
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
@fmt = private constant [4 x i8] c"%d\0A\00"

define void @print_i32(i32 %value) {
    %fmt_ptr = getelementptr [4 x i8], [4 x i8]* @fmt, i32 0, i32 0
    call i32 (i8*, ...) @printf(i8* %fmt_ptr, i32 %value)
    ret void
}

define i32 @test(){
entry:
%0 = add i32 12, 0
call void @print_i32(i32 %0)

ret i32 0
}

define i32 @main(){
entry:
%1 = sub i32 32, 64
%2 = add i32 %1, 0
call void @print_i32(i32 %2)

%3 = add i32 25, 0
call void @print_i32(i32 %3)

%4 = sub i32 60, 30
%5 = sub i32 4, 2
%6 = sub i32 %4, %5
%7 = add i32 %6, 0
call void @print_i32(i32 %7)

%8 = mul i32 300, 10
%9 = add i32 %8, 5
%10 = icmp eq i32 3005, %9
%11 = add i1 %10, 0
call void @print_i1(i1 %11)

%12 = icmp ne i32 69, 69
%13 = add i1 %12, 0
call void @print_i1(i1 %13)

ret i32 0
}
