use std::{collections::HashMap, error::Error};

use itertools::Itertools;

fn strip_comment(input: &str) -> &str {
    if let Some(idx) = input.find("//") {
        input.get(..idx).unwrap()
    } else {
        input
    }
}

pub fn preprocess(input: &str) -> Result<String, Box<dyn Error>> {
    let mut vars: HashMap::<&str, &str> = HashMap::new();
    let output = input
        .lines()
        .map(|l| strip_comment(l).trim_start())
        .map(|l| {
            if let Some(stripped) = l.strip_prefix("#define") {
                let (name, contents) = stripped.trim().split_once(char::is_whitespace).unwrap();
                vars.insert(name, contents.trim());
                return "".to_string()
            } else if let Some(name) = l.strip_prefix("#undef") {
                vars.remove(name.trim());
                return "".to_string()
            }
            let mut l = l.to_string();
            for (from, to) in vars.iter().sorted_unstable_by(|a, b| b.0.len().cmp(&a.0.len())) {
                l = l.replace(from, to)
            }
            l
        })
        .filter(|l| !l.is_empty())
        .join("\n");

    Ok(output)
}
