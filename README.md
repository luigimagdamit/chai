# Chai Programming Language

Chai is a **compiled programming language** implemented in **Rust**.  
It features:

- **Lexer** and **recursive-descent parsing** with **Pratt expressions**.
- A custom **code generation** module targeting **LLVM IR**, allowing compilation to multiple architectures such as **x86** and **ARM**.
- Support for **variables**, **control flow**, and **C ABI interoperability**.
- A lightweight **standard library written in C** for built-in functionality and system-level integration.

Chai’s syntax aims to be **concise like Go**, while future plans include **algebraic data types** inspired by functional programming languages.

---

## Quick Example

```c
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
````

---

## Build Instructions

### Prerequisites

* **Rust** (latest stable version)
* **Clang 18.1.8**

### Steps

```bash
# 1. Clone the repository
git clone https://github.com/luigimagdamit/chai.git
cd chai

# 2. Build the compiler
cargo build --release

# 3. Run the compiler
# Replace 'example.chai' with your source file
./target/release/chai example.chai
```

This will emit LLVM IR and produce an executable for your platform (x86, ARM, etc.) depending on your LLVM setup.

---

## Roadmap

* Algebraic data types
* Expanded standard library
* Additional backends and optimizations

---

## License

Specify your license here (e.g., MIT, Apache 2.0).

