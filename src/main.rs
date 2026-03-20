use full_moon::LuaVersion;
use full_moon::tokenizer::{Lexer, LexerResult};
use std::env::args;
use std::{fs, io, path, process};
use tracing::{info, error};

fn main() {
    tracing_subscriber::fmt()
        .with_ansi(true)
        .init();
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        error!("shortest argument. must require file-path");
        process::exit(1);
    }
    let script_path = path::PathBuf::from(&args[1]);
    if !script_path.exists() {
        error!("not found: '{}'", script_path.display());
        process::exit(1);
    }
    if !script_path.is_file() {
        error!("'{}' is not file", script_path.display());
        process::exit(1);
    }
    let content = match fs::read_to_string(&script_path) {
        Ok(content) => content,
        Err(err) => {
            error!("failed to read '{}': {}", script_path.display(), err);
            process::exit(1);
        }
    };
    let lexer = Lexer::new(&content, LuaVersion::lua51());
    let tokens = match lexer.collect() {
        LexerResult::Ok(tokens) => tokens,
        LexerResult::Fatal(errors) => {
            error!("Errors!");
            error!("{:#?}", errors);
            process::exit(1);
        }
        LexerResult::Recovered(tokens, errors) => {
            error!("Recovered!");
            error!("{:#?}", tokens);
            error!("{:#?}", errors);
            process::exit(1);
        }
    };
    info!("Tokenize Success!");
    for t in tokens {
        info!("{:#?}", t);
    }
    process::exit(0);
}
