
define i32 @main() {
entry:


	;	; pushing a new string on the stack ..."hello world!"
	%0 = getelementptr inbounds [14 x i8], [14 x i8]* @str0, i32 0, i32 0 ; llvm_retrieve_static_string

	;; print("hello world!");;
	call i32 (i8*, ...) @printf(i8* %0)

	;	; pushing a new string on the stack ..."beignets"
	%2 = getelementptr inbounds [10 x i8], [10 x i8]* @str1, i32 0, i32 0 ; llvm_retrieve_static_string

	;var breakfast : str = "beignets";
	%breakfast = alloca i8*
	store i8* %2, i8** %breakfast

	;	; pushing a new string on the stack ..."cafe au lait"
	%4 = getelementptr inbounds [14 x i8], [14 x i8]* @str2, i32 0, i32 0 ; llvm_retrieve_static_string

	;var beverage : str = "cafe au lait";
	%beverage = alloca i8*
	store i8* %4, i8** %beverage
	%breakfast_0 = load i8*, i8** %breakfast ; loading existing variable

	;; print(breakfast);;
	call i32 (i8*, ...) @printf(i8* %breakfast_0)
	%beverage_0 = load i8*, i8** %beverage ; loading existing variable

	;; print(beverage);;
	call i32 (i8*, ...) @printf(i8* %beverage_0)

	;var hungry : bool = true;
	%hungry = alloca i1
	store i1 1, i1* %hungry

	;var snacks : int = 10
	%snacks = alloca i32
	store i32 10, i32* %snacks
	br label %cond10
	
cond10:
	%snacks_0 = load i32, i32* %snacks ; loading existing variable
	%10 = icmp sgt i32 %snacks_0, 0

	;depth: 10
	br i1 %10, label %body10, label %exit10
	
body10:
	%snacks_1 = load i32, i32* %snacks ; loading existing variable
	%12 = sub i32 %snacks_1, 1
	store i32 %12, i32* %snacks
	 ; set symbol (symbol.rs)

	%hungry_0 = load i1, i1* %hungry ; loading existing variable
	%16 = icmp eq i1 %hungry_0, 1

	;	; pushing a new string on the stack ..."lets eat!"
	%18 = getelementptr inbounds [11 x i8], [11 x i8]* @str3, i32 0, i32 0 ; llvm_retrieve_static_string

	;; print("lets eat!");;
	call i32 (i8*, ...) @printf(i8* %18)

	;	; pushing a new string on the stack ..."nom"
	%20 = getelementptr inbounds [5 x i8], [5 x i8]* @str4, i32 0, i32 0 ; llvm_retrieve_static_string

	;; print("nom");;
	call i32 (i8*, ...) @printf(i8* %20)
	store i1 0, i1* %hungry
	 ; set symbol (symbol.rs)


	;	; pushing a new string on the stack ..."ima sleep"
	%22 = getelementptr inbounds [11 x i8], [11 x i8]* @str5, i32 0, i32 0 ; llvm_retrieve_static_string

	;; print("ima sleep");;
	call i32 (i8*, ...) @printf(i8* %22)

	;	; pushing a new string on the stack ..."zzz"
	%24 = getelementptr inbounds [5 x i8], [5 x i8]* @str6, i32 0, i32 0 ; llvm_retrieve_static_string

	;; print("zzz");;
	call i32 (i8*, ...) @printf(i8* %24)
	store i1 1, i1* %hungry
	 ; set symbol (symbol.rs)


	;depth: 1
	br i1 %16, label %then1, label %else1
	then1:
	br label %end1
	
else1:
	br label %end1
	
end1:
	br label %cond10
	
exit10:
	%26 = getelementptr inbounds [13 x i8], [13 x i8]* @str0, i32 0, i32 0 ; llvm_retrieve_static_string

	;var greeting : str = "hello world!";
	%greeting = alloca i8*
	store i8* %26, i8** %greeting

	;	; pushing a new string on the stack ..."this is chai"
	%28 = getelementptr inbounds [14 x i8], [14 x i8]* @str7, i32 0, i32 0 ; llvm_retrieve_static_string

	;var intro : str = "this is chai";
	%intro = alloca i8*
	store i8* %28, i8** %intro
	%greeting_0 = load i8*, i8** %greeting ; loading existing variable

	;; print(greeting);;
	call i32 (i8*, ...) @printf(i8* %greeting_0)
	%greeting_1 = load i8*, i8** %greeting ; loading existing variable

	;; print(greeting);;
	call i32 (i8*, ...) @printf(i8* %greeting_1)
	%intro_0 = load i8*, i8** %intro ; loading existing variable

	;; print(intro);;
	call i32 (i8*, ...) @printf(i8* %intro_0)
	%intro_1 = load i8*, i8** %intro ; loading existing variable

	;; print(intro);;
	call i32 (i8*, ...) @printf(i8* %intro_1)
	%greeting_2 = load i8*, i8** %greeting ; loading existing variable

	;; print(greeting);;
	call i32 (i8*, ...) @printf(i8* %greeting_2)

	;var a : int = 123
	%a = alloca i32
	store i32 123, i32* %a

	;var b : int = 69
	%b = alloca i32
	store i32 69, i32* %b
	%a_0 = load i32, i32* %a ; loading existing variable

	;; print(a);;
	call void @print_i32(i32 %a_0); signature from PrintVisitor

	%b_0 = load i32, i32* %b ; loading existing variable

	;; print(b);;
	call void @print_i32(i32 %b_0); signature from PrintVisitor

	%a_1 = load i32, i32* %a ; loading existing variable
	%b_1 = load i32, i32* %b ; loading existing variable
	%39 = icmp sgt i32 %a_1, %b_1

	;; print((a > b));;
	call void @print_i1(i1 %39); signature from PrintVisitor

	%a_2 = load i32, i32* %a ; loading existing variable
	%b_2 = load i32, i32* %b ; loading existing variable
	%40 = icmp slt i32 %a_2, %b_2

	;; print((a < b));;
	call void @print_i1(i1 %40); signature from PrintVisitor

	%a_3 = load i32, i32* %a ; loading existing variable
	%b_3 = load i32, i32* %b ; loading existing variable
	%41 = icmp eq i32 %a_3, %b_3

	;; print((a == b));;
	call void @print_i1(i1 %41); signature from PrintVisitor

	%a_4 = load i32, i32* %a ; loading existing variable
	%b_4 = load i32, i32* %b ; loading existing variable
	%42 = add i32 %b_4, 54
	%43 = icmp eq i32 %a_4, %42

	;; print((a == (b + 54)));;
	call void @print_i1(i1 %43); signature from PrintVisitor

	%44 = getelementptr inbounds [13 x i8], [13 x i8]* @str1, i32 0, i32 0 ; llvm_retrieve_static_string

	;; print("hello world!");;
	call i32 (i8*, ...) @printf(i8* %44)
	%46 = getelementptr inbounds [13 x i8], [13 x i8]* @str7, i32 0, i32 0 ; llvm_retrieve_static_string

	;; print("this is chai");;
	call i32 (i8*, ...) @printf(i8* %46)

	;	; pushing a new string on the stack ..."here is me declaring a variable"
	%48 = getelementptr inbounds [33 x i8], [33 x i8]* @str8, i32 0, i32 0 ; llvm_retrieve_static_string

	;; print("here is me declaring a variable");;
	call i32 (i8*, ...) @printf(i8* %48)

	;	; pushing a new string on the stack ..."this is the variable"
	%50 = getelementptr inbounds [22 x i8], [22 x i8]* @str9, i32 0, i32 0 ; llvm_retrieve_static_string

	;; print("this is the variable");;
	call i32 (i8*, ...) @printf(i8* %50)

	;var i : int = 24
	%i = alloca i32
	store i32 24, i32* %i
	%i_0 = load i32, i32* %i ; loading existing variable

	;; print(i);;
	call void @print_i32(i32 %i_0); signature from PrintVisitor

	%flag = alloca i1
	
	%55 = icmp slt i32 1, 2

	;; print((1 < 2));;
	call void @print_i1(i1 %55); signature from PrintVisitor

	%56 = icmp sgt i32 3, 1

	;; print((3 > 1));;
	call void @print_i1(i1 %56); signature from PrintVisitor

	%57 = icmp eq i32 2, 3

	;; print((2 == 3));;
	call void @print_i1(i1 %57); signature from PrintVisitor


	;; print(true);;
	call void @print_i1(i1 1); signature from PrintVisitor


	;; print(false);;
	call void @print_i1(i1 0); signature from PrintVisitor


	;	; pushing a new string on the stack ..."ok shutting down now"
	%58 = getelementptr inbounds [22 x i8], [22 x i8]* @str10, i32 0, i32 0 ; llvm_retrieve_static_string

	;var seppuku : str = "ok shutting down now";
	%seppuku = alloca i8*
	store i8* %58, i8** %seppuku
	%seppuku_0 = load i8*, i8** %seppuku ; loading existing variable

	;; print(seppuku);;
	call i32 (i8*, ...) @printf(i8* %seppuku_0)

	;var ae : int = 0
	%ae = alloca i32
	store i32 0, i32* %ae

	;var be : int = 0
	%be = alloca i32
	store i32 0, i32* %be
	br label %cond63
	
cond63:
	%ae_0 = load i32, i32* %ae ; loading existing variable
	%63 = icmp slt i32 %ae_0, 5

	;depth: 63
	br i1 %63, label %body63, label %exit63
	
body63:
	%ae_1 = load i32, i32* %ae ; loading existing variable

	;; print(ae);;
	call void @print_i32(i32 %ae_1); signature from PrintVisitor

	%ae_2 = load i32, i32* %ae ; loading existing variable
	%66 = add i32 %ae_2, 1
	store i32 %66, i32* %ae
	 ; set symbol (symbol.rs)

	br label %cond67
	
cond67:
	%be_0 = load i32, i32* %be ; loading existing variable
	%67 = icmp slt i32 %be_0, 5

	;depth: 67
	br i1 %67, label %body67, label %exit67
	
body67:
	%be_1 = load i32, i32* %be ; loading existing variable

	;; print(be);;
	call void @print_i32(i32 %be_1); signature from PrintVisitor

	%be_2 = load i32, i32* %be ; loading existing variable
	%70 = add i32 %be_2, 1
	store i32 %70, i32* %be
	 ; set symbol (symbol.rs)

	br label %cond67
	
exit67:
	store i32 0, i32* %be
	 ; set symbol (symbol.rs)


	;	; pushing a new string on the stack ..."yup"
	%71 = getelementptr inbounds [5 x i8], [5 x i8]* @str11, i32 0, i32 0 ; llvm_retrieve_static_string

	;; print("yup");;
	call i32 (i8*, ...) @printf(i8* %71)
	br label %cond63
	
exit63:

	ret i32 0 ; llvm_main_close
}


	;String Constants
@str2 = private unnamed_addr constant [14 x i8] c"cafe au lait\0A\00", align 1
@str11 = private unnamed_addr constant [5 x i8] c"yup\0A\00", align 1
@str1 = private unnamed_addr constant [10 x i8] c"beignets\0A\00", align 1
@str0 = private unnamed_addr constant [14 x i8] c"hello world!\0A\00", align 1
@str5 = private unnamed_addr constant [11 x i8] c"ima sleep\0A\00", align 1
@str7 = private unnamed_addr constant [14 x i8] c"this is chai\0A\00", align 1
@str3 = private unnamed_addr constant [11 x i8] c"lets eat!\0A\00", align 1
@str9 = private unnamed_addr constant [22 x i8] c"this is the variable\0A\00", align 1
@str10 = private unnamed_addr constant [22 x i8] c"ok shutting down now\0A\00", align 1
@str6 = private unnamed_addr constant [5 x i8] c"zzz\0A\00", align 1
@str8 = private unnamed_addr constant [33 x i8] c"here is me declaring a variable\0A\00", align 1
@str4 = private unnamed_addr constant [5 x i8] c"nom\0A\00", align 1


@fmt = private constant [4 x i8] c"%d\0A\00"
declare i32 @printf(i8*, ...)
define void @print_i32(i32 %value) {
    %fmt_ptr = getelementptr [4 x i8], [4 x i8]* @fmt, i32 0, i32 0
    call i32 (i8*, ...) @printf(i8* %fmt_ptr, i32 %value)
    ret void
}
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