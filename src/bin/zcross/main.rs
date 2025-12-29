use ticompile::{assembler::AssemblerOptions, compile_file};

fn main() {
    let options = AssemblerOptions::from_name("ZCROSS");

    let display = compile_file("./src/bin/zcross/zcross.tibasic", options);
    println!("{display}");
}
