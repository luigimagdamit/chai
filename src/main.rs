
mod scanner;  // Import the Scanner module
mod parser;
mod common;
mod llvm;

use common::common::{PARSE_DECLARATION_MODE, PARSE_EXPRESSION_MODE};
use llvm::llvm_print::{llvm_fmt_string_int, llvm_main_close, llvm_main_start, llvm_print_define, llvm_print_i32_define};
use parser::parser::Parser;
use std::io::{self, Write};
use std::fs::{self, File};
use std::process::{Command, exit};
use std::time::{SystemTime, UNIX_EPOCH};
fn repl() -> io::Result<()>{
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read REPL");
        let source = input.trim();
        if source == "exit" {
            ()
        }
        
        let parser = &mut Parser::init_parser(source);
        parser.compilation += &llvm_fmt_string_int();
        parser.compilation += &llvm_print_define();
        parser.compilation += &llvm_print_i32_define();
        let compile_start = SystemTime::now();
        parser.compilation += "\n";
        parser.compilation += &llvm_main_start();
        parser.compilation += "\n";
        parser.compile(); // warmup
        parser.compilation += &llvm_main_close();
        io::stdout().flush().unwrap();
        let mut file = File::create("jit.ll")?;
        // println!("{}", parser.compilation);
        file.write_all(parser.compilation.as_bytes())?;
        
        let output = Command::new("clang")
            .arg("jit.ll")
            .arg("-w")
            .arg("-o")
            .arg("jit")
            .output();
        let compile_end = SystemTime::now();
        match output {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    println!("\x1b[32mSuccessfully compiled!\x1b[0m{}", stdout);
                    let compile_time = compile_end.duration_since(UNIX_EPOCH).unwrap().as_millis() - compile_start.duration_since(UNIX_EPOCH).unwrap().as_millis();
                    println!("\x1b[33mJIT Compile Time: \x1b[0m{}ms\n", compile_time);
                    let run_output = Command::new("./jit").output();
                    match run_output {
                        Ok(out) => {
                            if out.status.success() {
                                let stdout = String::from_utf8_lossy(&out.stdout);
                                println!("\x1b[33mYou said: \x1b[0m{}", source);
                                println!("\x1b[32mChai says: \x1b[0m{}", stdout);
                            } else {
                                panic!()
                            }
                        },
                        _ => {}
                    }
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("{}", stderr);
                    exit(1);
                }
            }
            _ => ()
        }
    }
}
fn main() {

    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        if PARSE_EXPRESSION_MODE { let _ = repl(); }
        else { panic!("[ExprModeError] PARSE_EXPRESSION_MODE flag is not enabled") }
    } else {
        match &args[1] {

            s if s.is_ascii()=> {
                if !PARSE_DECLARATION_MODE { panic!("[DeclrModeError] PARSE_DECLARATION_MODE flag is not enabled")}
                let contents = fs::read_to_string(s).unwrap();
                let parser = &mut Parser::init_parser(&contents);
                parser.compile();
            }
            _ => {}
        }
        
    }
}
