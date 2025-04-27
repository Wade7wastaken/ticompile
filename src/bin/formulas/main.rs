use formula_builder::build_formulas;
use ticompile::{
    compile::{compile_from_bytes, CompilerOptions},
    preprocessor::preprocess,
    tokens::{load_token_json, TokenTrie},
};

mod formula_builder;

fn main() {
    let program = build_formulas();

    let processed = preprocess(&program).unwrap();

    let token_json = load_token_json().unwrap();
    let token_trie = TokenTrie::load_tokens(token_json).unwrap();

    let tokens = token_trie.tokenize(&processed);

    println!("{}", tokens.display());

    let options = CompilerOptions {
        file_name: "FORMULAS.8xp",
        program_name: "FORMULAS",
        comment: "comment",
    };

    compile_from_bytes(tokens.into_bytes(), options).unwrap();
}
