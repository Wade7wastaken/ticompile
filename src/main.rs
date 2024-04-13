use std::{fs::File, io::Write};

fn generate_header(body_len: u16) -> Vec<u8> {
    // https://gist.github.com/SimonEast/244a0fd04526ea1acbec2e2ceb2e7924
    let mut header = Vec::with_capacity(55);

    header.extend_from_slice(b"**TI83F*"); // signature
    header.extend_from_slice(&[0x1A, 0x0A]); // signature_part_2
    header.push(0x0A); // mystery_byte
    header.extend_from_slice(&[0; 42]); // comment
    header.extend_from_slice(&(body_len + 19).to_le_bytes()); // meta_and_body_length

    assert!(header.len() == 55);

    header
}

fn generate_meta_data(body_len: u16, program_name: &[u8; 8]) -> Vec<u8> {
    let mut meta_data = Vec::with_capacity(19);

    let body_and_checksum_length = &(body_len + 2).to_le_bytes();

    meta_data.push(0x0D); // flag
    meta_data.push(0x00); // unknown
    meta_data.extend_from_slice(body_and_checksum_length); // body_and_checksum_length
    meta_data.push(0x05); // file_type
    meta_data.extend_from_slice(program_name); // program_name
    meta_data.push(0x00); // version
    meta_data.push(0x00); // is_archived
    meta_data.extend_from_slice(body_and_checksum_length); // body_and_checksum_length_2
    meta_data.extend_from_slice(&body_len.to_le_bytes()); // body_length

    assert!(meta_data.len() == 19);

    meta_data
}

fn generate_data() -> Vec<u8> {
    let mut data = Vec::new();

    data.extend_from_slice(&[0xDE, 0x2A, 0x48, 0x49]);

    data
}

type ChecksumIntermediate = u32;

fn sum_of_u8s(v: &Vec<u8>) -> ChecksumIntermediate {
    v.iter().map(|&x| x as ChecksumIntermediate).sum()
}

fn generate_checksum(meta_data: &Vec<u8>, data: &Vec<u8>) -> u16 {
    ((sum_of_u8s(meta_data) + sum_of_u8s(data)) & 0xFFFF) as u16
}

fn write_file() -> std::io::Result<()> {
    let mut file = File::create("ABCDEFGH.8xp")?;

    let data = generate_data();
    let header = generate_header(data.len() as u16);
    let meta_data = generate_meta_data(data.len() as u16, b"ABCDEFGH");
    let checksum = generate_checksum(&meta_data, &data);

    file.write_all(&header)?;
    file.write_all(&meta_data)?;
    file.write_all(&data)?;
    file.write_all(&checksum.to_le_bytes())?;

    Ok(())
}

fn main() {
    write_file().unwrap();
}
