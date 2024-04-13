pub fn generate_data() -> Vec<u8> {
    let mut data = Vec::new();

    data.extend_from_slice(&[0xDE, 0x2A, 0x48, 0x49]);

    data
}
