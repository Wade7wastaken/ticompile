use std::fs;

use assembler::{assemble_from_bytes, AssemblerOptions};
use tokens::{load_token_json, TokenTrie};

pub mod assembler;
pub mod header;
pub mod metadata;
pub mod tokens;

pub fn compile_file(file_path: &str, assembler_options: AssemblerOptions) -> String {
    let program_str = fs::read_to_string(file_path).unwrap();
    compile_str(&program_str, assembler_options)
}

pub fn compile_str(program_str: &str, assembler_options: AssemblerOptions) -> String {
    let token_json = load_token_json().unwrap();
    let token_trie = TokenTrie::load_tokens(token_json).unwrap();

    let tokens = token_trie.tokenize(program_str);

    let display = tokens.display();

    assemble_from_bytes(tokens.into_bytes(), assembler_options).unwrap();

    display
}
