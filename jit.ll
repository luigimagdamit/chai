@fmt = private constant [4 x i8] c"%d\0A\00"declare i32 @printf(i8*, ...)
define void @print_i32(i32 %value) {
    %fmt_ptr = getelementptr [4 x i8], [4 x i8]* @fmt, i32 0, i32 0
    call i32 (i8*, ...) @printf(i8* %fmt_ptr, i32 %value)
    ret void
}define void @print_i1(i1 %b) {
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
define i32 @main() {
entry:

%greeting = alloca i8*
getelementptr inbounds [14 x i8], [14 x i8]* @str0, i32 0, i32 0

%2 = getelementptr inbounds [13 x i8], [13 x i8]* @str0, i32 0, i32 0
;LLVM Register for String @ ExprCount 2(variable.rs)

store i8* %2, i8** %greeting
%intro = alloca i8*
getelementptr inbounds [14 x i8], [14 x i8]* @str1, i32 0, i32 0

%5 = getelementptr inbounds [13 x i8], [13 x i8]* @str1, i32 0, i32 0
;LLVM Register for String @ ExprCount 5(variable.rs)

store i8* %5, i8** %intro

%greeting_0 = load i8*, i8** %greeting

%6 = getelementptr inbounds [13 x i8], [13 x i8]* @str0, i32 0, i32 0
;LLVM Register for String @ ExprCount 6(variable.rs)

call i32 (i8*, ...) @printf(i8* %6)
; Auto generated by LlvmCallPrint (print.rs)

%greeting_1 = load i8*, i8** %greeting

%8 = getelementptr inbounds [13 x i8], [13 x i8]* @str0, i32 0, i32 0
;LLVM Register for String @ ExprCount 8(variable.rs)

call i32 (i8*, ...) @printf(i8* %8)
; Auto generated by LlvmCallPrint (print.rs)

%intro_0 = load i8*, i8** %intro

%10 = getelementptr inbounds [13 x i8], [13 x i8]* @str1, i32 0, i32 0
;LLVM Register for String @ ExprCount 10(variable.rs)

call i32 (i8*, ...) @printf(i8* %10)
; Auto generated by LlvmCallPrint (print.rs)

%intro_1 = load i8*, i8** %intro

%12 = getelementptr inbounds [13 x i8], [13 x i8]* @str1, i32 0, i32 0
;LLVM Register for String @ ExprCount 12(variable.rs)

call i32 (i8*, ...) @printf(i8* %12)
; Auto generated by LlvmCallPrint (print.rs)

%greeting_2 = load i8*, i8** %greeting

%14 = getelementptr inbounds [13 x i8], [13 x i8]* @str0, i32 0, i32 0
;LLVM Register for String @ ExprCount 14(variable.rs)

call i32 (i8*, ...) @printf(i8* %14)
; Auto generated by LlvmCallPrint (print.rs)
%a = alloca i32
store i32 123, i32* %a
%b = alloca i32
store i32 69, i32* %b
%19 = add i32 %a_0, 0
call void @print_i32(i32 %19); Auto generated by LlvmCallPrint (i 32)(print.rs)
%21 = add i32 %b_0, 0
call void @print_i32(i32 %21); Auto generated by LlvmCallPrint (i 32)(print.rs)
%24 = icmp ugt i32 %a_1, %b_1
%25 = add i1 %24, 0
call void @print_i1(i1 %25); Auto generated by LlvmCallPrint (i1) (print.rs)
%28 = icmp ult i32 %a_2, %b_2
%29 = add i1 %28, 0
call void @print_i1(i1 %29); Auto generated by LlvmCallPrint (i1) (print.rs)
%32 = icmp eq i32 %a_3, %b_3
%33 = add i1 %32, 0
call void @print_i1(i1 %33); Auto generated by LlvmCallPrint (i1) (print.rs)
%36 = add i32 %b_4, 54
%37 = icmp eq i32 %a_4, %36
%38 = add i1 %37, 0
call void @print_i1(i1 %38); Auto generated by LlvmCallPrint (i1) (print.rs)
%40 = getelementptr inbounds [13 x i8], [13 x i8]* @str1, i32 0, i32 0
;LLVM Register for String @ ExprCount 40(variable.rs)

call i32 (i8*, ...) @printf(i8* %40)
; Auto generated by LlvmCallPrint (print.rs)
%43 = getelementptr inbounds [13 x i8], [13 x i8]* @str2, i32 0, i32 0
;LLVM Register for String @ ExprCount 43(variable.rs)

call i32 (i8*, ...) @printf(i8* %43)
; Auto generated by LlvmCallPrint (print.rs)
getelementptr inbounds [33 x i8], [33 x i8]* @str2, i32 0, i32 0

%46 = getelementptr inbounds [32 x i8], [32 x i8]* @str2, i32 0, i32 0
;LLVM Register for String @ ExprCount 46(variable.rs)

call i32 (i8*, ...) @printf(i8* %46)
; Auto generated by LlvmCallPrint (print.rs)
getelementptr inbounds [22 x i8], [22 x i8]* @str3, i32 0, i32 0

%49 = getelementptr inbounds [21 x i8], [21 x i8]* @str3, i32 0, i32 0
;LLVM Register for String @ ExprCount 49(variable.rs)

call i32 (i8*, ...) @printf(i8* %49)
; Auto generated by LlvmCallPrint (print.rs)
%i = alloca i32
store i32 24, i32* %i
%53 = add i32 %i_0, 0
call void @print_i32(i32 %53); Auto generated by LlvmCallPrint (i 32)(print.rs)
%flag = alloca i1
%54 = icmp ult i32 1, 2
%55 = add i1 %54, 0
call void @print_i1(i1 %55); Auto generated by LlvmCallPrint (i1) (print.rs)
%56 = icmp ugt i32 3, 1
%57 = add i1 %56, 0
call void @print_i1(i1 %57); Auto generated by LlvmCallPrint (i1) (print.rs)
%58 = icmp eq i32 2, 3
%59 = add i1 %58, 0
call void @print_i1(i1 %59); Auto generated by LlvmCallPrint (i1) (print.rs)
%61 = add i1 1, 0
call void @print_i1(i1 %61); Auto generated by LlvmCallPrint (i1) (print.rs)
%63 = add i1 0, 0
call void @print_i1(i1 %63); Auto generated by LlvmCallPrint (i1) (print.rs)
%seppuku = alloca i8*
getelementptr inbounds [22 x i8], [22 x i8]* @str4, i32 0, i32 0

%66 = getelementptr inbounds [21 x i8], [21 x i8]* @str4, i32 0, i32 0
;LLVM Register for String @ ExprCount 66(variable.rs)

store i8* %66, i8** %seppuku

%seppuku_0 = load i8*, i8** %seppuku

%67 = getelementptr inbounds [21 x i8], [21 x i8]* @str4, i32 0, i32 0
;LLVM Register for String @ ExprCount 67(variable.rs)

call i32 (i8*, ...) @printf(i8* %67)
; Auto generated by LlvmCallPrint (print.rs)

ret i32 0
}@str0 = private unnamed_addr constant [14 x i8] c"hello world!\0A\00", align 1@str1 = private unnamed_addr constant [14 x i8] c"this is chai\0A\00", align 1@str3 = private unnamed_addr constant [22 x i8] c"this is the variable\0A\00", align 1@str4 = private unnamed_addr constant [22 x i8] c"ok shutting down now\0A\00", align 1@str2 = private unnamed_addr constant [33 x i8] c"here is me declaring a variable\0A\00", align 1