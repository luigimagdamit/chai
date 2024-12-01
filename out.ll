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
%greeting = alloca i8*
%1 = getelementptr inbounds [14 x i8], [14 x i8]* @str0, i32 0, i32 0
store i8* %1, i8** %greeting
%intro = alloca i8*
%3 = getelementptr inbounds [14 x i8], [14 x i8]* @str1, i32 0, i32 0
store i8* %3, i8** %intro
%greeting_0 = load i8*, i8** %greeting
call i32 (i8*, ...) @printf(i8* %greeting_0)
%intro_0 = load i8*, i8** %intro
call i32 (i8*, ...) @printf(i8* %intro_0)
%a = alloca i32
store i32 123, i32* %a
%b = alloca i32
store i32 69, i32* %b
%a_0 = load i32, i32* %a
%b_0 = load i32, i32* %b
%14 = icmp ugt i32 %a_0, %b_0
call void @print_i1(i1 %14)

%a_1 = load i32, i32* %a
%b_1 = load i32, i32* %b
%18 = icmp ult i32 %a_1, %b_1
call void @print_i1(i1 %18)

%a_2 = load i32, i32* %a
%b_2 = load i32, i32* %b
%22 = icmp eq i32 %a_2, %b_2
call void @print_i1(i1 %22)

%a_3 = load i32, i32* %a
%b_3 = load i32, i32* %b
%26 = add i32 %b_3, 54
%27 = icmp eq i32 %a_3, %26
call void @print_i1(i1 %27)

%seppuku = alloca i8*
%30 = getelementptr inbounds [22 x i8], [22 x i8]* @str2, i32 0, i32 0
store i8* %30, i8** %seppuku
%seppuku_0 = load i8*, i8** %seppuku
call i32 (i8*, ...) @printf(i8* %seppuku_0)
ret i32 0
}
@str2 = private unnamed_addr constant [22 x i8] c"ok shutting down now\0A\00", align 1
@str0 = private unnamed_addr constant [14 x i8] c"hello world!\0A\00", align 1
@str1 = private unnamed_addr constant [14 x i8] c"this is chai\0A\00", align 1
