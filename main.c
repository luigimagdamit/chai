#include <stdio.h>
#include <stdbool.h>

int main() {
    // Generated IR code (translated to C)

	;	; pushing a new string on the stack ..."hello world!"
	reg0 = str_0;

	;; print("hello world!");;
	printf("%s\n", %0);

	;	; pushing a new string on the stack ..."beignets"
	reg2 = str_2;

	;var breakfast : str = "beignets";
	char* breakfast;
	breakfast = %2;

	;	; pushing a new string on the stack ..."cafe au lait"
	reg4 = str_4;

	;var beverage : str = "cafe au lait";
	char* beverage;
	beverage = %4;
	breakfast_0 = breakfast;

	;; print(breakfast);;
	printf("%s\n", %breakfast_0);
	beverage_0 = beverage;

	;; print(beverage);;
	printf("%s\n", %beverage_0);

	;var hungry : bool = true;
	bool hungry;
	hungry = 1;

	;var snacks : int = 10
	int snacks;
	snacks = 10;
	goto cond10;
	cond10:
	snacks_0 = snacks;
	reg10 = %snacks_0 > 0;

	;depth: 10
	if (reg10) goto body10; else goto exit10;
	body10:
	snacks_1 = snacks;
	reg12 = %snacks_1 - 1;
	store i32 %12, i32* %snacks
	 ; set symbol (symbol.rs)

	hungry_0 = hungry;
	reg16 = %hungry_0 == 1;

	;	; pushing a new string on the stack ..."lets eat!"
	reg18 = str_18;

	;; print("lets eat!");;
	printf("%s\n", %18);

	;	; pushing a new string on the stack ..."nom"
	reg20 = str_20;

	;; print("nom");;
	printf("%s\n", %20);
	store i1 0, i1* %hungry
	 ; set symbol (symbol.rs)


	;	; pushing a new string on the stack ..."ima sleep"
	reg22 = str_22;

	;; print("ima sleep");;
	printf("%s\n", %22);

	;	; pushing a new string on the stack ..."zzz"
	reg24 = str_24;

	;; print("zzz");;
	printf("%s\n", %24);
	store i1 1, i1* %hungry
	 ; set symbol (symbol.rs)


	;depth: 1
	if (reg16) goto then1; else goto else1;
	then1:
	goto end1;
	else1:
	goto end1;
	end1:
	goto cond10;
	exit10:
	reg26 = str_26;

	;var greeting : str = "hello world!";
	char* greeting;
	greeting = %26;

	;	; pushing a new string on the stack ..."this is chai"
	reg28 = str_28;

	;var intro : str = "this is chai";
	char* intro;
	intro = %28;
	greeting_0 = greeting;

	;; print(greeting);;
	printf("%s\n", %greeting_0);
	greeting_1 = greeting;

	;; print(greeting);;
	printf("%s\n", %greeting_1);
	intro_0 = intro;

	;; print(intro);;
	printf("%s\n", %intro_0);
	intro_1 = intro;

	;; print(intro);;
	printf("%s\n", %intro_1);
	greeting_2 = greeting;

	;; print(greeting);;
	printf("%s\n", %greeting_2);

	;var a : int = 123
	int a;
	a = 123;

	;var b : int = 69
	int b;
	b = 69;
	a_0 = a;

	;; print(a);;
	printf("%d\n", %a_0);
	b_0 = b;

	;; print(b);;
	printf("%d\n", %b_0);
	a_1 = a;
	b_1 = b;
	reg39 = %a_1 > %b_1;

	;; print((a > b));;
	printf("%s\n", %39 ? "true" : "false");
	a_2 = a;
	b_2 = b;
	reg40 = %a_2 < %b_2;

	;; print((a < b));;
	printf("%s\n", %40 ? "true" : "false");
	a_3 = a;
	b_3 = b;
	reg41 = %a_3 == %b_3;

	;; print((a == b));;
	printf("%s\n", %41 ? "true" : "false");
	a_4 = a;
	b_4 = b;
	reg42 = %b_4 + 54;
	reg43 = %a_4 == %42;

	;; print((a == (b + 54)));;
	printf("%s\n", %43 ? "true" : "false");
	reg44 = str_44;

	;; print("hello world!");;
	printf("%s\n", %44);
	reg46 = str_46;

	;; print("this is chai");;
	printf("%s\n", %46);

	;	; pushing a new string on the stack ..."here is me declaring a variable"
	reg48 = str_48;

	;; print("here is me declaring a variable");;
	printf("%s\n", %48);

	;	; pushing a new string on the stack ..."this is the variable"
	reg50 = str_50;

	;; print("this is the variable");;
	printf("%s\n", %50);

	;var i : int = 24
	int i;
	i = 24;
	i_0 = i;

	;; print(i);;
	printf("%d\n", %i_0);
	bool flag;
	
	reg55 = 1 < 2;

	;; print((1 < 2));;
	printf("%s\n", %55 ? "true" : "false");
	reg56 = 3 > 1;

	;; print((3 > 1));;
	printf("%s\n", %56 ? "true" : "false");
	reg57 = 2 == 3;

	;; print((2 == 3));;
	printf("%s\n", %57 ? "true" : "false");

	;; print(true);;
	printf("%s\n", 1 ? "true" : "false");

	;; print(false);;
	printf("%s\n", 0 ? "true" : "false");

	;	; pushing a new string on the stack ..."ok shutting down now"
	reg58 = str_58;

	;var seppuku : str = "ok shutting down now";
	char* seppuku;
	seppuku = %58;
	seppuku_0 = seppuku;

	;; print(seppuku);;
	printf("%s\n", %seppuku_0);

	;var ae : int = 0
	int ae;
	ae = 0;

	;var be : int = 0
	int be;
	be = 0;
	goto cond63;
	cond63:
	ae_0 = ae;
	reg63 = %ae_0 < 5;

	;depth: 63
	if (reg63) goto body63; else goto exit63;
	body63:
	ae_1 = ae;

	;; print(ae);;
	printf("%d\n", %ae_1);
	ae_2 = ae;
	reg66 = %ae_2 + 1;
	store i32 %66, i32* %ae
	 ; set symbol (symbol.rs)

	goto cond67;
	cond67:
	be_0 = be;
	reg67 = %be_0 < 5;

	;depth: 67
	if (reg67) goto body67; else goto exit67;
	body67:
	be_1 = be;

	;; print(be);;
	printf("%d\n", %be_1);
	be_2 = be;
	reg70 = %be_2 + 1;
	store i32 %70, i32* %be
	 ; set symbol (symbol.rs)

	goto cond67;
	exit67:
	store i32 0, i32* %be
	 ; set symbol (symbol.rs)


	;	; pushing a new string on the stack ..."yup"
	reg71 = str_71;

	;; print("yup");;
	printf("%s\n", %71);
	goto cond63;
	exit63:

	;String Constants
static const char str_8 = "here is me declaring a variable";
static const char str_9 = "this is the variable";
static const char str_5 = "ima sleep";
static const char str_4 = "nom";
static const char str_7 = "this is chai";
static const char str_6 = "zzz";
static const char str_1 = "beignets";
static const char str_0 = "hello world!";
static const char str_10 = "ok shutting down now";
static const char str_11 = "yup";
static const char str_3 = "lets eat!";
static const char str_2 = "cafe au lait";

    return 0;
}
