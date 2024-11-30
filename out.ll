declare i32 @printf(i8*, ...)
@fmt = private constant [4 x i8] c"%d\0A\00"

define void @print_i32(i32 %value) {
    %fmt_ptr = getelementptr [4 x i8], [4 x i8]* @fmt, i32 0, i32 0
    call i32 (i8*, ...) @printf(i8* %fmt_ptr, i32 %value)
    ret void
}

define i32 @test(){
entry:
%0 = add i32 12, 0
call void @print_i32(i32 %0)

ret i32 0
}

define i32 @main(){
entry:
%1 = sub i32 32, 64
%2 = add i32 %1, 0
call void @print_i32(i32 %2)

%3 = add i32 25, 0
call void @print_i32(i32 %3)

%4 = sub i32 60, 30
%5 = sub i32 4, 2
%6 = sub i32 %4, %5
%7 = add i32 %6, 0
call void @print_i32(i32 %7)

%8 = add i32 3005, 0
call void @print_i32(i32 %8)

ret i32 0
}
