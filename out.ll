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

%2 = getelementptr inbounds [14 x i8], [14 x i8]* @str1, i32 0, i32 0

store i8* %2, i8** %intro

%greeting_0 = load i8*, i8** %greeting

call i32 (i8*, ...) @printf(i8* %greeting_0)

%greeting_1 = load i8*, i8** %greeting

call i32 (i8*, ...) @printf(i8* %greeting_1)

%intro_0 = load i8*, i8** %intro

call i32 (i8*, ...) @printf(i8* %intro_0)

%intro_1 = load i8*, i8** %intro

call i32 (i8*, ...) @printf(i8* %intro_1)

%greeting_2 = load i8*, i8** %greeting

call i32 (i8*, ...) @printf(i8* %greeting_2)
%a = alloca i32

store i32 123, i32* %a
%b = alloca i32

store i32 69, i32* %b
%a_0 = load i32, i32* %a
%8 = add i32 %a_0, 0

call void @print_i32(i32 %8)


%b_0 = load i32, i32* %b
%10 = add i32 %b_0, 0

call void @print_i32(i32 %10)


%a_1 = load i32, i32* %a
%b_1 = load i32, i32* %b
%13 = icmp ugt i32 %a_1, %b_1

%14 = add i1 %13, 0
%a_2 = load i32, i32* %a
%b_2 = load i32, i32* %b
%17 = icmp ult i32 %a_2, %b_2

%18 = add i1 %17, 0
%a_3 = load i32, i32* %a
%b_3 = load i32, i32* %b
%21 = icmp eq i32 %a_3, %b_3

%22 = add i1 %21, 0
%a_4 = load i32, i32* %a
%b_4 = load i32, i32* %b
%25 = add i32 %b_4, 54

%26 = icmp eq i32 %a_4, %25

%27 = add i1 %26, 0

%29 = getelementptr inbounds [13 x i8], [13 x i8]* @str0, i32 0, i32 0

call i32 (i8*, ...) @printf(i8* %29)


%31 = getelementptr inbounds [13 x i8], [13 x i8]* @str1, i32 0, i32 0

call i32 (i8*, ...) @printf(i8* %31)


%33 = getelementptr inbounds [33 x i8], [33 x i8]* @str2, i32 0, i32 0

call i32 (i8*, ...) @printf(i8* %33)


%35 = getelementptr inbounds [22 x i8], [22 x i8]* @str3, i32 0, i32 0

call i32 (i8*, ...) @printf(i8* %35)

%i = alloca i32

store i32 24, i32* %i
%i_0 = load i32, i32* %i
%37 = add i32 %i_0, 0

call void @print_i32(i32 %37)


%flag = alloca i1

%38 = icmp ult i32 1, 2

%39 = add i1 %38, 0
%40 = icmp ugt i32 3, 1

%41 = add i1 %40, 0
%42 = icmp eq i32 2, 3

%43 = add i1 %42, 0
%45 = add i1 1, 0
%47 = add i1 0, 0
%seppuku = alloca i8*

%49 = getelementptr inbounds [22 x i8], [22 x i8]* @str4, i32 0, i32 0

store i8* %49, i8** %seppuku

%seppuku_0 = load i8*, i8** %seppuku

call i32 (i8*, ...) @printf(i8* %seppuku_0)
ret i32 0
}
@str2 = private unnamed_addr constant [33 x i8] c"here is me declaring a variable\0A\00", align 1
@str1 = private unnamed_addr constant [14 x i8] c"this is chai\0A\00", align 1
@str0 = private unnamed_addr constant [14 x i8] c"hello world!\0A\00", align 1
@str4 = private unnamed_addr constant [22 x i8] c"ok shutting down now\0A\00", align 1
@str3 = private unnamed_addr constant [22 x i8] c"this is the variable\0A\00", align 1
