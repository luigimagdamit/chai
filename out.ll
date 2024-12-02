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

define i32 @main(){
entry:
%a = alloca i32

store i32 2, i32* %a
%a_0 = load i32, i32* %a
%1 = add i32 %a_0, 0

call void @print_i32(i32 %1)


%a_1 = load i32, i32* %a
%3 = mul i32 %a_1, 2
store i32 %3, i32* %a

%a_2 = load i32, i32* %a
%5 = add i32 %a_2, 0

call void @print_i32(i32 %5)


%a_3 = load i32, i32* %a
%7 = mul i32 %a_3, 2
store i32 %7, i32* %a

%a_4 = load i32, i32* %a
%9 = add i32 %a_4, 0

call void @print_i32(i32 %9)


%a_5 = load i32, i32* %a
%11 = mul i32 %a_5, 2
store i32 %11, i32* %a

%a_6 = load i32, i32* %a
%13 = add i32 %a_6, 0

call void @print_i32(i32 %13)


%a_7 = load i32, i32* %a
%15 = mul i32 %a_7, 2
store i32 %15, i32* %a

%a_8 = load i32, i32* %a
%17 = add i32 %a_8, 0

call void @print_i32(i32 %17)


%a_9 = load i32, i32* %a
%19 = mul i32 %a_9, 2
store i32 %19, i32* %a

%a_10 = load i32, i32* %a
%21 = add i32 %a_10, 0

call void @print_i32(i32 %21)


%a_11 = load i32, i32* %a
%23 = mul i32 %a_11, 2
store i32 %23, i32* %a

%a_12 = load i32, i32* %a
%25 = add i32 %a_12, 0

call void @print_i32(i32 %25)


ret i32 0
}
