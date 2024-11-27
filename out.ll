declare i32 @printf(i8*, ...)
@fmt = private constant [4 x i8] c"%d\0A\00"
define i32 @0() {
entry:
    ret i32 3005
}
define i32 @main() {
entry:
    %result = call i32 @0()

    %fmt_ptr = getelementptr [4 x i8], [4 x i8]* @fmt, i32 0, i32 0
    call i32 @printf(i8* %fmt_ptr, i32 %result)
    ret i32 0
}
