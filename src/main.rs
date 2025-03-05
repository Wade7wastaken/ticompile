use ticompile::{
    compile::{compile_from_bytes, CompilerOptions},
    formulas::build_formulas,
    preprocessor::preprocess,
    tokens::{load_token_json, TokenTrie},
};

fn main() {
    let program = build_formulas();
    let processed = preprocess(&program).unwrap();
    // println!("{processed}");

    let token_json = load_token_json().unwrap();
    let token_trie = TokenTrie::load_tokens(token_json).unwrap();

    let tokens = token_trie.tokenize(&processed);

    println!("{}", tokens.display());

    compile_from_bytes(tokens.into_bytes(), CompilerOptions::default()).unwrap();
}
