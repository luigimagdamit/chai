Chai is a compiled programming language implemented in Rust that features a lexer, recursive descent and Pratt parsing, and a custom code generation module targeting LLVM IR. It can compile to many different architectures such as x86 or ARM processors. 

The compiler supports variables, control flow, and C ABI interoperability. It includes a custom standard library written in C, offering built-in functionality and system-level integration.

Chai focuses on maintaining concise code similar to Golang, but plans to integrate algebraic data types similar to those found in functional programming languages.

Here is a quick sneak peek:

```go
fn main() int {
    print("hello world!");

    var breakfast: str = "beignets";
    var beverage: str = "cafe au lait";

    print(breakfast);
    print(beverage);

    var hungry: bool = true;

    var snacks: int = 10;

    while (snacks != 0) {
        snacks = snacks - 1;

        if (hungry == true) {
            print("lets eat!");
            print("nom");
            hungry = false;
        } else {
            print("ima sleep");
            print("zzz");
            hungry = true;
        }

    }
}
```




