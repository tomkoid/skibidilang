use crate::{
    lexer::{Token, TokenKind},
    loader::Loader,
    BUILD_DIR,
};

pub fn tokens_to_asm(tokens: &Vec<Token>) -> String {
    let mut output = String::new();

    output.push_str("global _start\n_start:\n");
    for (i, token) in tokens.iter().enumerate() {
        match token.kind {
            TokenKind::Return => {
                if i + 1 < tokens.len() && tokens[i + 1].kind == TokenKind::Int {
                    if i + 2 < tokens.len() && tokens[i + 2].kind == TokenKind::Semi {
                        output.push_str("    mov rax, 60\n");
                        output.push_str(&format!(
                            "    mov rdi, {}\n",
                            tokens[i + 1].value.clone().unwrap()
                        ));
                        output.push_str("    syscall\n");
                    }
                }
            }
            _ => {} // Ignore other token types for now
        }
    }
    output
}

pub fn create_build_dir(build_dir: &str) -> Result<(), std::io::Error> {
    if std::path::Path::new(build_dir).exists() {
        std::fs::remove_dir_all(build_dir)?;
        std::fs::create_dir_all(build_dir)?;
    } else {
        std::fs::create_dir_all(build_dir)?;
    }

    Ok(())
}

pub fn make_executable(asm_output: &str, file: &str) {
    let mut l = Loader::new("Assembling code...".to_string());

    // write asm to file
    std::fs::write(format!("{}/output.s", BUILD_DIR), asm_output).expect("Failed to write file");

    let output = std::process::Command::new("nasm")
        .arg("-felf64")
        .arg("-o")
        .arg(format!("{}/output.o", BUILD_DIR))
        .arg(format!("{}/output.s", BUILD_DIR))
        .output()
        .expect("Failed to execute nasm");

    if output.status.success() {
        l.stop_success();
    } else {
        l.stop_error_msg("Failed to assemble code.".to_string());
        std::process::exit(1);
    }

    let mut l = Loader::new("Linking code...".to_string());

    let output = std::process::Command::new("ld")
        .arg("-o")
        .arg("output")
        .arg(format!("{}/output.o", BUILD_DIR))
        .output()
        .expect("Failed to execute ld");

    if output.status.success() {
        l.stop_success();
    } else {
        l.stop_error_msg("Failed to link code.".to_string());
        std::process::exit(1);
    }

    println!("Compiled {} successfully.", file);
}
