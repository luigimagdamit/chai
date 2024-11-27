declare i32 @printf(i8*, ...)
@fmt = private constant [4 x i8] c"%d\0A\00"
define void @print_i32(i32 %value) {
        %fmt_ptr = getelementptr [4 x i8], [4 x i8]* @fmt, i32 0, i32 0
        call i32 (i8*, ...) @printf(i8* %fmt_ptr, i32 %value)
        ret void
    }
define i32 @0() {
entry:
    ret i32 35001
}
define i32 @main() {
entry:
    %result = call i32 @0()

    call void @print_i32(i32 %result)
    ret i32 0
}
