mod body;
mod checksum;
mod compile;
mod header;
mod meta_data;
mod tokens;

fn main() {
    let tokens = tokens::process_tokens();
    let mut program = "Disp \"HELLO WORLD\"";
    let mut bytes = vec![];
    while !program.is_empty() {
        let (mut tokens, left) = tokens::tokenize(program, &tokens);
        bytes.append(&mut tokens);
        program = left;
    }
    compile::from_bytes(bytes, compile::DEFAULT_OPTIONS).unwrap();
}
