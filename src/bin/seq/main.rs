use ticompile::{assembler::AssemblerOptions, compile_file};

fn main() {
    let options = AssemblerOptions::from_name("SEQ");

    let display = compile_file("./src/bin/seq/seq.tibasic", options);
    println!("{display}");
}
