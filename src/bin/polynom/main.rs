use ticompile::{assembler::AssemblerOptions, compile_file};

fn main() {
    let options = AssemblerOptions::from_name("POLYNOM");
    let display = compile_file("./src/bin/polynom/polynom.tibasic", options);
    println!("{display}");
}
