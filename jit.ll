@fmt = private constant [4 x i8] c"%d\0A\00"declare i32 @printf(i8*, ...)
define void @print_i32(i32 %value) {
    %fmt_ptr = getelementptr [4 x i8], [4 x i8]* @fmt, i32 0, i32 0
    call i32 (i8*, ...) @printf(i8* %fmt_ptr, i32 %value)
    ret void
}
define i32 @main() {
entry:

%a = alloca i8*
%1 = getelementptr inbounds [3 x i8], [3 x i8]* @str0, i32 0, i32 0

store i8* %1, i8** %a
%b = alloca i8*

%a_0 = load i8*, i8** %a

%1 = %a_0
store i8* %1, i8** %b

ret i32 0
}@str0 = private unnamed_addr constant [3 x i8] c"a\0A\00", align 1