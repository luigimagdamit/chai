declare i32 @printf(i8*, ...)
@fmt = private constant [4 x i8] c"%d\0A\00"

define void @print_i32(i32 %value) {
    %fmt_ptr = getelementptr [4 x i8], [4 x i8]* @fmt, i32 0, i32 0
    call i32 (i8*, ...) @printf(i8* %fmt_ptr, i32 %value)
    ret void
}

define i32 @main(){
entry:
%0 = sub i32 32, 64
%1 = add i32 %0, 0
call void @print_i32(i32 %1)

%2 = add i32 25, 0
call void @print_i32(i32 %2)

%3 = sub i32 60, 30
%4 = sub i32 4, 2
%5 = sub i32 %3, %4
%6 = add i32 %5, 0
call void @print_i32(i32 %6)

ret i32 0
}
