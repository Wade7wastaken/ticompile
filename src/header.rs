pub fn generate_header(body_len: u16, comment: &str) -> Vec<u8> {
    // https://gist.github.com/SimonEast/244a0fd04526ea1acbec2e2ceb2e7924
    let mut header = Vec::with_capacity(55);

    header.extend_from_slice(b"**TI83F*"); // signature
    header.extend_from_slice(&[0x1A, 0x0A]); // signature_part_2
    header.push(0x0A); // mystery_byte
    header.extend_from_slice(&process_comment(comment)); // comment
    header.extend_from_slice(&(body_len + 19).to_le_bytes()); // meta_and_body_length

    assert!(header.len() == 55);

    header
}

fn process_comment(comment: &str) -> [u8; 42] {
    let comment = if comment.len() >= 42 {
        let new_comment = &comment[..42];
        eprintln!(
            "Truncating comment to 42 characters: \"{}\" -> \"{}\"",
            comment, new_comment
        );
        new_comment
    } else {
        comment
    };

    // this assert should never be hit, but just in case
    assert!(comment.len() <= 42);

    let mut comment_bytes = [0; 42];

    comment_bytes[..comment.len()].copy_from_slice(comment.as_bytes());

    comment_bytes
}
