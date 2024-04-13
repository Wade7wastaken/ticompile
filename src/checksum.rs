type ChecksumIntermediate = u32;

fn sum_of_u8s(v: &Vec<u8>) -> ChecksumIntermediate {
    v.iter().map(|&x| x as ChecksumIntermediate).sum()
}

pub fn generate_checksum(meta_data: &Vec<u8>, data: &Vec<u8>) -> u16 {
    ((sum_of_u8s(meta_data) + sum_of_u8s(data)) & 0xFFFF) as u16
}
