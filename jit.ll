@fmt = private constant [4 x i8] c"%d\0A\00"declare i32 @printf(i8*, ...)
define void @print_i32(i32 %value) {
    %fmt_ptr = getelementptr [4 x i8], [4 x i8]* @fmt, i32 0, i32 0
    call i32 (i8*, ...) @printf(i8* %fmt_ptr, i32 %value)
    ret void
}
define i32 @main() {
entry:

%greeting = alloca i8*

getelementptr inbounds [14 x i8], [14 x i8]* @str0, i32 0, i32 0
%1 = getelementptr inbounds [14 x i8], [14 x i8]* @str0, i32 0, i32 0
store i8* %1, i8** %greeting%intro = alloca i8*

getelementptr inbounds [14 x i8], [14 x i8]* @str1, i32 0, i32 0
%2 = getelementptr inbounds [14 x i8], [14 x i8]* @str1, i32 0, i32 0
store i8* %2, i8** %intro
%greeting_0 = load i8*, i8** %greeting
call i32 (i8*, ...) @printf(i8* %greeting_0)
%greeting_1 = load i8*, i8** %greeting
call i32 (i8*, ...) @printf(i8* %greeting_1)