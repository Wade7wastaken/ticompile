use compile::{compile_from_bytes, CompilerOptions};
use tokens::{load_tokens, tokenize};

mod compile;
mod header;
mod metadata;
mod tokens;

fn main() {
    let tokens = load_tokens().unwrap();
    let program = "Disp \"HELLO WORLD\"";
    let bytes = tokenize(program, &tokens);
    compile_from_bytes(bytes, CompilerOptions::default()).unwrap();
}
