use ticompile::{
    compile::{compile_from_bytes, CompilerOptions},
    preprocessor::preprocess,
    tokens::{load_token_json, TokenTrie},
};

fn main() {
    let program_str = include_str!("./zcross.8xp");
    let processed = preprocess(program_str).unwrap();

    // println!("{processed}\n\n");

    let token_json = load_token_json().unwrap();
    let token_trie = TokenTrie::load_tokens(token_json).unwrap();

    let tokens = token_trie.tokenize(&processed);

    // println!("{:?}", tokens.clone().into_bytes());

    println!("{}", tokens.display());

    let options = CompilerOptions {
        file_name: "ZCROSS.8xp",
        program_name: "ZCROSS",
        comment: "comment",
    };

    compile_from_bytes(tokens.into_bytes(), options).unwrap();
}
