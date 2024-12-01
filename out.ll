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
%13 = add i32 %a_0, 0
call void @print_i32(i32 %13)

%b_0 = load i32, i32* %b
%15 = add i32 %b_0, 0
call void @print_i32(i32 %15)

%a_1 = load i32, i32* %a
%b_1 = load i32, i32* %b
%18 = icmp ugt i32 %a_1, %b_1
%19 = add i1 %18, 0
call void @print_i1(i1 %19)

%a_2 = load i32, i32* %a
%b_2 = load i32, i32* %b
%22 = icmp ult i32 %a_2, %b_2
%23 = add i1 %22, 0
call void @print_i1(i1 %23)

%a_3 = load i32, i32* %a
%b_3 = load i32, i32* %b
%26 = icmp eq i32 %a_3, %b_3
%27 = add i1 %26, 0
call void @print_i1(i1 %27)

%a_4 = load i32, i32* %a
%b_4 = load i32, i32* %b
%30 = add i32 %b_4, 54
%31 = icmp eq i32 %a_4, %30
%32 = add i1 %31, 0
call void @print_i1(i1 %32)

%34 = getelementptr inbounds [13 x i8], [13 x i8]* @str0, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %34)
%37 = getelementptr inbounds [13 x i8], [13 x i8]* @str1, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %37)
%40 = getelementptr inbounds [33 x i8], [33 x i8]* @str2, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %40)
%43 = getelementptr inbounds [22 x i8], [22 x i8]* @str3, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %43)
%i = alloca i32
store i32 24, i32* %i
%i_0 = load i32, i32* %i
%47 = add i32 %i_0, 0
call void @print_i32(i32 %47)

%flag = alloca i1
%48 = icmp ult i32 1, 2
%49 = add i1 %48, 0
call void @print_i1(i1 %49)

%50 = icmp ugt i32 3, 1
%51 = add i1 %50, 0
call void @print_i1(i1 %51)

%52 = icmp eq i32 2, 3
%53 = add i1 %52, 0
call void @print_i1(i1 %53)

%55 = add i1 1, 0
call void @print_i1(i1 %55)

%57 = add i1 0, 0
call void @print_i1(i1 %57)

%seppuku = alloca i8*
%59 = getelementptr inbounds [22 x i8], [22 x i8]* @str4, i32 0, i32 0
store i8* %59, i8** %seppuku
%seppuku_0 = load i8*, i8** %seppuku
call i32 (i8*, ...) @printf(i8* %seppuku_0)
ret i32 0
}
@str0 = private unnamed_addr constant [14 x i8] c"hello world!\0A\00", align 1
@str4 = private unnamed_addr constant [22 x i8] c"ok shutting down now\0A\00", align 1
@str3 = private unnamed_addr constant [22 x i8] c"this is the variable\0A\00", align 1
@str1 = private unnamed_addr constant [14 x i8] c"this is chai\0A\00", align 1
@str2 = private unnamed_addr constant [33 x i8] c"here is me declaring a variable\0A\00", align 1
