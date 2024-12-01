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
%1 = getelementptr inbounds [14 x i8], [14 x i8]* @str0, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %1)
%4 = getelementptr inbounds [14 x i8], [14 x i8]* @str1, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %4)
%7 = getelementptr inbounds [33 x i8], [33 x i8]* @str2, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %7)
%i = alloca i32
store i32 24, i32* %i
%i_val = load i32, i32* %i
call void @print_i32(i32 %i_val)
%flag = alloca i1
%10 = icmp ult i32 1, 2
call void @print_i1(i1 %10)

%12 = icmp ugt i32 3, 1
call void @print_i1(i1 %12)

%14 = icmp eq i32 2, 3
call void @print_i1(i1 %14)

%17 = getelementptr inbounds [24 x i8], [24 x i8]* @str3, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %17)
ret i32 0
}
@str2 = private unnamed_addr constant [33 x i8] c"here is me declaring a variable\0A\00", align 1
@str0 = private unnamed_addr constant [14 x i8] c"hello world!\0A\00", align 1
@str1 = private unnamed_addr constant [14 x i8] c"this is chai\0A\00", align 1
@str3 = private unnamed_addr constant [24 x i8] c"okay shutting down now\0A\00", align 1
