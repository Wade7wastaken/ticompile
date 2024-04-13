use std::{fs::File, io::Write};

pub struct Options<'a> {
    file_name: &'a str,
    comment: &'a str,
    program_name: &'a str,
}

// default compile options
pub const DEFAULT_OPTIONS: Options = Options {
    file_name: "PROGRAM.8xp",
    comment: "Generated with ticompile",
    program_name: "PROGRAM",
};

pub fn from_bytes(data: Vec<u8>, options: Options) -> std::io::Result<()> {
    let mut file = File::create(options.file_name)?;

    let header = crate::header::generate(data.len() as u16, options.comment);
    let meta_data = crate::meta_data::generate(data.len() as u16, options.program_name);
    let checksum = crate::checksum::generate_checksum(&meta_data, &data);

    file.write_all(&header)?;
    file.write_all(&meta_data)?;
    file.write_all(&data)?;
    file.write_all(&checksum.to_le_bytes())?;

    Ok(())
}
