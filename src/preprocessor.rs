use std::error::Error;

use itertools::Itertools;

fn strip_comment(input: &str) -> &str {
    if let Some(idx) = input.find("//") {
        input.get(..idx).unwrap()
    } else {
        input
    }
}

pub fn preprocess(input: &str) -> Result<String, Box<dyn Error>> {
    let output = input
        .lines()
        .map(|l| strip_comment(l).trim())
        .filter(|l| !l.is_empty())
        .join("\n");

    Ok(output)
}
