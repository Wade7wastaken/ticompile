use compile::{compile_from_bytes, CompilerOptions};
use formulas::build_formulas;
use preprocessor::preprocess;
use tokens::{load_tokens, tokenize};

mod compile;
mod header;
mod metadata;
mod preprocessor;
mod tokens;
mod formulas;

fn main() {
    let formulas = build_formulas();
    println!("{formulas}");
    // let program = include_str!("../test.tibasic");
    // let processed = preprocess(program).unwrap();
    // println!("{processed}");


    // let tokens = load_tokens().unwrap();
    // let program = "Disp \"HELLO WORLD\"";
    // let bytes = tokenize(program, &tokens);
    // compile_from_bytes(bytes, CompilerOptions::default()).unwrap();
}
