use std::{
    fs::File,
    io::{self, Write},
};

use crate::{header::generate_header, metadata::generate_metadata};

#[derive(Debug, Clone)]
pub struct AssemblerOptions {
    pub file_name: String,
    pub comment: String,
    pub program_name: String,
}

impl AssemblerOptions {
    pub fn from_name(name: &str) -> Self {
        let program_name = name.to_ascii_uppercase();
        let file_name = format!("{program_name}.8xp");
        Self {
            file_name,
            program_name,
            ..Default::default()
        }
    }
    pub fn from_name_with_comment(name: &str, comment: &str) -> Self {
        let program_name = name.to_ascii_uppercase();
        let file_name = format!("{program_name}.8xp");
        Self {
            file_name,
            program_name,
            comment: comment.to_string(),
        }
    }
}

impl Default for AssemblerOptions {
    fn default() -> Self {
        Self {
            file_name: "PROGRAM.8xp".to_string(),
            comment: "Generated with ticompile".to_string(),
            program_name: "PROGRAM".to_string(),
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

pub fn assemble_from_bytes(data: Vec<u8>, options: AssemblerOptions) -> io::Result<()> {
    let mut file = File::create(options.file_name)?;

    let header = generate_header(data.len() as u16, &options.comment);
    let metadata = generate_metadata(data.len() as u16, &options.program_name);
    let checksum = generate_checksum(&metadata, &data).to_le_bytes();

    file.write_all(&header)?;
    file.write_all(&metadata)?;
    file.write_all(&data)?;
    file.write_all(&checksum)?;

    Ok(())
}
