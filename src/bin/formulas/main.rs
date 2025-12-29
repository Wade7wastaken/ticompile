use formula_builder::build_formulas;
use ticompile::{assembler::AssemblerOptions, compile_str};

mod formula_builder;

fn main() {
    let program = build_formulas();

    let options = AssemblerOptions::from_name("FORMULAS");

    let display = compile_str(&program, options);
    println!("{}", display);
}
