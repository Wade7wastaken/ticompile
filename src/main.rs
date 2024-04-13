mod body;
mod checksum;
mod compile;
mod header;
mod meta_data;
mod tokens;

fn main() {
    compile::from_bytes(body::generate_data(), compile::DEFAULT_OPTIONS).unwrap();
}
