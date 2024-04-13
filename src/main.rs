mod checksum;
mod header;
mod meta_data;

use std::{fs::File, io::Write};
fn generate_data() -> Vec<u8> {
    let mut data = Vec::new();

    data.extend_from_slice(&[0xDE, 0x2A, 0x48, 0x49]);

    data
}

fn write_file() -> std::io::Result<()> {
    let mut file = File::create("ABCDEFGH.8xp")?;

    let data = generate_data();
    let header = header::generate(data.len() as u16, "Comment");
    let meta_data = meta_data::generate(data.len() as u16, "TEST");
    let checksum = checksum::generate_checksum(&meta_data, &data);

    file.write_all(&header)?;
    file.write_all(&meta_data)?;
    file.write_all(&data)?;
    file.write_all(&checksum.to_le_bytes())?;

    Ok(())
}

fn main() {
    write_file().unwrap();
}
