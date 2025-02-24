use std::{
    fs::File,
    io::{self, Write},
};

use crate::{header::generate_header, metadata::generate_metadata};

#[derive(Debug, Clone)]
pub struct CompilerOptions<'a> {
    file_name: &'a str,
    comment: &'a str,
    program_name: &'a str,
}

impl Default for CompilerOptions<'_> {
    fn default() -> Self {
        Self {
            file_name: "PROGRAM.8xp",
            comment: "Generated with ticompile",
            program_name: "PROGRAM",
        }
    }
}

type N = u32;

fn sum_of_u8s(v: &[u8]) -> N {
    v.iter().map(|x| *x as N).sum()
}

fn generate_checksum(metadata: &[u8], data: &[u8]) -> u16 {
    ((sum_of_u8s(metadata) + sum_of_u8s(data)) & 0xFFFF) as u16
}

pub fn compile_from_bytes(data: Vec<u8>, options: CompilerOptions) -> io::Result<()> {
    let mut file = File::create(options.file_name)?;

    let header = generate_header(data.len() as u16, options.comment);
    let metadata = generate_metadata(data.len() as u16, options.program_name);
    let checksum = generate_checksum(&metadata, &data).to_le_bytes();

    file.write_all(&header)?;
    file.write_all(&metadata)?;
    file.write_all(&data)?;
    file.write_all(&checksum)?;

    Ok(())
}
