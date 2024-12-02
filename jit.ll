@fmt = private constant [4 x i8] c"%d\0A\00"declare i32 @printf(i8*, ...)
define void @print_i32(i32 %value) {
    %fmt_ptr = getelementptr [4 x i8], [4 x i8]* @fmt, i32 0, i32 0
    call i32 (i8*, ...) @printf(i8* %fmt_ptr, i32 %value)
    ret void
}
define i32 @main() {
entry:

%0 = mul i32 69, 420
%1 = add i32 %0, 0
call void @print_i32(i32 %1)



ret i32 0
}