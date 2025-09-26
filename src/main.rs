
mod scanner;  // Import the Scanner module
mod parser;
mod common;
mod llvm;
mod jit;
mod codegen;

use std::env;
use parser::parser::Parser;
use std::io::{self, Write};
use jit::compile::jit_compile;
use codegen::backend_config::{IRBackend, init_backend_config, get_current_backend};
use std::str::FromStr;

fn chai_title() -> String {
    String::from(r#"
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⠿⠿⠿⢿⣿⣿⣽⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⠿⣿⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠁⠀⣠⣄⡀⠀⠙⢿⣿⣿⣿⣿⣿⣿⣿⡿⠋⠁⠀⠀⠀⠈⠻⣿⣻⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣟⣿⣿⡿⠁⠀⣰⣾⣿⣿⣿⣦⡀⠀⠛⠛⠛⠛⠛⠛⠁⠀⠀⣰⣿⣿⣿⣦⠀⠀⠺⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⡿⠀⠀⣼⣿⣿⣿⣿⣿⣿⣿⣤⣤⢤⢰⢠⡄⡄⣤⣴⣿⣿⣿⣿⣿⣿⣷⡄⠀⢹⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⠿⠧⠄⠸⠿⠿⠿⠿⠿⠿⢿⣿⣿⣿⣿⣿⡿⡷⢟⠿⠿⠿⠿⠿⠿⠿⠿⠿⣿⣤⣼⣿⣿⣿⣿⣿⣿
⣿⠛⠻⠿⣿⠀⢀⣀⣀⣤⣤⣄⣀⣀⣀⣠⡍⠈⣿⣿⣿⣿⣿⣿⡟⠀⣀⣀⣀⣀⣀⣀⣀⣀⣀⡀⠀⣹⣿⡿⠟⠻⣿
⣿⣦⣤⣀⣀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠿⠿⠿⠿⠻⠟⠇⠀⢹⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠋⠉⠀⣀⣴⣿
⠋⠉⠀⠀⢹⠀⢸⣿⣿⠟⠻⣿⣿⣿⣿⣿⡇⠀⣠⣤⣤⣤⣤⣤⣤⠀⢸⣿⣿⣿⣿⡿⠛⢿⣿⣿⠀⠀⠀⠙⠛⠛⢿
⣷⣾⣿⣿⣿⠀⠘⠿⠿⠤⠤⠿⠛⠛⠛⠛⠁⠀⣿⣻⣿⣿⣿⣿⣿⠀⠈⠛⠛⠛⠿⠧⠤⠼⠿⠇⠀⣼⣶⣶⣶⣤⣼
⣿⣿⣿⣿⣿⡆⠀⠀⣔⣯⡵⣠⣤⣤⣤⣤⣤⡀⠉⠉⠉⠉⡀⠀⠈⢠⣤⣤⣤⣤⣤⣤⣤⣤⣤⡀⠀⣿⣿⣿⣿⣿⣿
⣿⣿⡿⣫⣥⣲⣤⣴⣼⣿⣇⣏⠟⠻⠿⠿⢿⣿⠀⠀⠂⠀⠛⠀⢠⣿⣿⣿⣿⣿⣿⣿⡿⠿⠋⠀⣠⣿⣿⣿⣿⣿⣿
⣿⣟⠀⢿⣿⣿⣿⣿⣼⣿⣿⣿⠇⢤⣤⣀⣠⣿⣦⣤⣤⣶⣤⣤⣾⣿⣿⣿⣿⣿⡁⠀⢀⡀⠀⠀⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣇⠈⢿⣿⣿⣿⣷⣿⣿⣿⠄⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠛⠁⢀⣴⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣤⡙⢻⡿⣿⠿⢛⠁⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣽⣿⣿⣿⣿⣿⣿⣿⣿⡀⠀⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣷⣶⣶⣶⣶⣿⠀⠸⡿⠛⠿⠛⠛⠛⠃⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⣿⠇⠀⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡆⠀⠁⢀⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣴⣦⡀⠀⠀⣠⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣯⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
"#)
}

fn repl() -> io::Result<()>{
    println!("\x1b[93m{}\nchai v0.00.1 REPL", chai_title());
    println!("\x1b[93mtype something!\x1b[0m");
    let mut body = "".to_string();
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read REPL");
        let source = input.trim();
        if source == "exit" {
            println!("\x1b[93mchai takes a nap...\x1b[0m");
            std::process::exit(0);
        }
        let jit = jit_compile(source);
        body += &jit.unwrap();
        println!("{}", body)
        
    }
}
fn print_help() {
    println!("Chai Programming Language Compiler");
    println!("Usage: chai [OPTIONS] [FILE]");
    println!();
    println!("OPTIONS:");
    println!("  --backend <BACKEND>  Specify IR backend: llvm (default), c");
    println!("  --help, -h           Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("  chai program.chai                # Compile with LLVM backend");
    println!("  chai --backend c program.chai    # Compile with C backend");
    println!("  chai --backend llvm program.chai # Compile with LLVM backend");
}

fn compile_with_backend(source_file: &str, backend: IRBackend) {
    // Initialize the backend configuration
    init_backend_config(backend);

    println!("Using {} backend", backend);

    let contents = std::fs::read_to_string(source_file)
        .unwrap_or_else(|_| panic!("Could not read source file: {}", source_file));

    let _ = &mut Parser::init_parser(&contents);

    match backend {
        IRBackend::LLVM => {
            let _ = jit_compile(&contents);
        }
        IRBackend::C => {
            compile_to_c(&contents, source_file);
        }
    }
}

fn compile_to_c(source: &str, source_file: &str) {
    println!("Compiling {} to C...", source_file);

    let parser = Parser::init_parser(source);

    // Generate the C code using the parser's compilation output
    let generated_c_code = generate_c_wrapper(&parser.compilation);

    // Write to C file
    let c_file = source_file.replace(".chai", ".c");
    match std::fs::write(&c_file, generated_c_code) {
        Ok(_) => {
            println!("Generated C file: {}", c_file);

            // Try to compile the C file
            compile_and_run_c(&c_file);
        }
        Err(e) => {
            eprintln!("Failed to write C file {}: {}", c_file, e);
        }
    }
}

fn generate_c_wrapper(ir_code: &str) -> String {
    format!(r#"#include <stdio.h>
#include <stdbool.h>

int main() {{
    // Generated IR code (translated to C)
{}
    return 0;
}}
"#, ir_code)
}

fn compile_and_run_c(c_file: &str) {
    use std::process::Command;

    let executable = c_file.replace(".c", "");

    // Compile the C file
    println!("Compiling C file with gcc...");
    let compile_result = Command::new("gcc")
        .args(&[c_file, "-o", &executable])
        .output();

    match compile_result {
        Ok(output) => {
            if output.status.success() {
                println!("C compilation successful: {}", executable);

                // Run the executable
                println!("Running compiled program...");
                let run_result = Command::new(&format!("./{}", executable))
                    .output();

                match run_result {
                    Ok(run_output) => {
                        if !run_output.stdout.is_empty() {
                            println!("Output:");
                            print!("{}", String::from_utf8_lossy(&run_output.stdout));
                        }
                        if !run_output.stderr.is_empty() {
                            println!("Error output:");
                            print!("{}", String::from_utf8_lossy(&run_output.stderr));
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to run executable: {}", e);
                    }
                }
            } else {
                println!("C compilation failed:");
                print!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            eprintln!("Failed to run gcc: {}", e);
        }
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        // Default to LLVM backend for REPL
        init_backend_config(IRBackend::LLVM);
        repl();
        return;
    }

    let mut backend = IRBackend::LLVM;
    let mut source_file: Option<String> = None;
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--backend" => {
                if i + 1 < args.len() {
                    backend = IRBackend::from_str(&args[i + 1])
                        .unwrap_or_else(|e| panic!("{}", e));
                    i += 2;
                } else {
                    eprintln!("Error: --backend requires an argument");
                    print_help();
                    std::process::exit(1);
                }
            }
            "--help" | "-h" => {
                print_help();
                return;
            }
            arg if !arg.starts_with("--") => {
                source_file = Some(arg.to_string());
                i += 1;
            }
            _ => {
                eprintln!("Error: Unknown argument: {}", args[i]);
                print_help();
                std::process::exit(1);
            }
        }
    }

    if let Some(file) = source_file {
        compile_with_backend(&file, backend);
    } else {
        eprintln!("Error: No source file specified");
        print_help();
        std::process::exit(1);
    }
}
