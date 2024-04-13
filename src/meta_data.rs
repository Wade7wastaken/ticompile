pub fn generate(body_len: u16, program_name: &str) -> Vec<u8> {
    let mut meta_data = Vec::with_capacity(19);

    let body_and_checksum_length = &(body_len + 2).to_le_bytes();

    meta_data.push(0x0D); // flag
    meta_data.push(0x00); // unknown
    meta_data.extend_from_slice(body_and_checksum_length); // body_and_checksum_length
    meta_data.push(0x05); // file_type
    meta_data.extend_from_slice(&process_program_name(program_name)); // program_name
    meta_data.push(0x00); // version
    meta_data.push(0x00); // is_archived
    meta_data.extend_from_slice(body_and_checksum_length); // body_and_checksum_length_2
    meta_data.extend_from_slice(&body_len.to_le_bytes()); // body_length

    assert!(meta_data.len() == 19);

    meta_data
}

fn process_program_name(name: &str) -> [u8; 8] {
    let name = if name.len() >= 8 {
        let new_name = &name[..8];
        eprintln!(
            "Truncating program name to 8 characters: \"{}\" -> \"{}\"",
            name, new_name
        );
        new_name
    } else {
        name
    };

    // this assert should never be hit, but just in case
    assert!(name.len() <= 8);

    let mut name_bytes = [0; 8];

    name_bytes[..name.len()].copy_from_slice(name.as_bytes());

    name_bytes
}
