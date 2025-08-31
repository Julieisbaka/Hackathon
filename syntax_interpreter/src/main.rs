mod lexer;
mod parser;
mod ast;
mod evaluator;

use std::env;
use std::fs;
use evaluator::Env;

fn main() {
    let mut args: env::Args = env::args();
    let _program: Option<String> = (&mut args).next();
    let Some(path) = (&mut args).next() else {
        eprintln!("usage: syntax_interpreter <file.ms>");
        std::process::exit(1);
    };

    let src: String = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("failed to read {}: {}", path, e);
            std::process::exit(1);
        }
    };

    let mut env: Env = Env::with_builtins();
    let mut buffer: String = String::new();
    for line in (&*src).lines() {
        (&mut buffer).push_str(line);
        (&mut buffer).push('\n');
        if !is_input_complete(&**&buffer) { continue; }
        if (&*buffer).trim().is_empty() { (&mut buffer).clear(); continue; }
        let tokens: Vec<lexer::Token> = lexer::tokenize(&**&buffer);
        let ast: ast::AstNode = parser::parse(&**&tokens);
        let _ = evaluator::eval(&ast, &mut env);
        (&mut buffer).clear();
    }
    // In case file does not end with newline but last form is complete
    if !(&*buffer).trim().is_empty() && is_input_complete(&**&buffer) {
        let tokens: Vec<lexer::Token> = lexer::tokenize(&**&buffer);
        let ast: ast::AstNode = parser::parse(&**&tokens);
        let _ = evaluator::eval(&ast, &mut env);
    }
}


fn is_input_complete(src: &str) -> bool {
    let bytes = src.as_bytes();
    let mut i: usize = 0usize;
    let mut paren: i32 = 0i32;
    let mut brace: i32 = 0i32;
    let mut bracket: i32 = 0i32;
    let mut in_string: bool = false;
    let mut in_doc: bool = false;
    while i < bytes.len() {
        // handle docstrings
        if in_doc {
            if i + 2 < bytes.len() && bytes[i] == b'"' && bytes[i + 1] == b'"' && bytes[i + 2] == b'"' {
                in_doc = false;
                i += 3;
                continue;
            }
            i += 1;
            continue;
        }
        if in_string {
            if bytes[i] == b'\\' {
                i = (i + 2).min(bytes.len());
                continue;
            }
            if bytes[i] == b'"' {
                in_string = false;
                i += 1;
                continue;
            }
            i += 1;
            continue;
        }
        // comments
        if bytes[i] == b'#' {
            while i < bytes.len() && bytes[i] != b'\n' { i += 1; }
            continue;
        }
        // start of docstring
        if i + 2 < bytes.len() && bytes[i] == b'"' && bytes[i + 1] == b'"' && bytes[i + 2] == b'"' {
            in_doc = true;
            i += 3;
            continue;
        }
        // start of string
        if bytes[i] == b'"' {
            in_string = true;
            i += 1;
            continue;
        }
        match bytes[i] {
            b'(' => paren += 1,
            b')' => { if paren > 0 { paren -= 1; } },
            b'{' => brace += 1,
            b'}' => { if brace > 0 { brace -= 1; } },
            b'[' => bracket += 1,
            b']' => { if bracket > 0 { bracket -= 1; } },
            _ => {}
        }
        i += 1;
    }
    !in_string && !in_doc && paren == 0 && brace == 0 && bracket == 0
}
