use std::fs::{self, File};
use std::io::{self, Write};
use std::process::{Command, exit};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::llvm::llvm_print::{llvm_fmt_string_int, llvm_main_close, llvm_main_start, llvm_print_bool_declare, llvm_print_define, llvm_print_i32_define, llvm_memcpy_declare};
use crate::parser::parser::Parser;
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

pub fn jit_compile(source: &str) -> io::Result<String>{
    let parser = &mut Parser::init_parser(source);

        let compile_start = SystemTime::now();
        parser.compilation += "\n";
        parser.compilation += &llvm_main_start();
        parser.compilation += "\n";
        parser.compile(); // warmup
        parser.compilation += &llvm_fmt_string_int();
        parser.compilation += &llvm_memcpy_declare();
        parser.compilation += &llvm_print_define();
        parser.compilation += &llvm_print_i32_define();
        parser.compilation += &llvm_print_bool_declare();
        
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
                    let out_clone = stdout.clone();
                    println!("\x1b[33mYou said: \x1b[0m{}", source);
                    println!("\x1b[32mSuccessfully compiled!\x1b[0m{}", stdout);
                    let compile_time = compile_end.duration_since(UNIX_EPOCH).unwrap().as_millis() - compile_start.duration_since(UNIX_EPOCH).unwrap().as_millis();
                    println!("\x1b[33mJIT Compile Time: \x1b[0m{}ms\n", compile_time);
                    let run_output = Command::new("./jit").output();
                    match run_output {
                        Ok(out) => {
                            if out.status.success() {
                                let stdout = String::from_utf8_lossy(&out.stdout);
                                let res = stdout.to_string();
                                
                                
                                println!("\x1b[32m{} \x1b[0m", chai_title());
                                println!("\x1b[32mchai says: \x1b[0m{}", &stdout);
                                Ok(res)
                            } else {
                                panic!()
                            }
                            
                        },
                        _ => panic!()
                    }
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("{}", stderr);
                    exit(1);
                }
            }
            _ => panic!()
        }
}
