use std::{collections::HashMap, error::Error, fmt::Display};

#[derive(Debug, Clone)]
enum PreProcessErrorKind {
    EmptyDefine,
    RedefinedVar { name: String, orig_line: usize },
    EmptyUndef,
}

impl Display for PreProcessErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            PreProcessErrorKind::EmptyDefine => "Empty define".to_string(),
            PreProcessErrorKind::RedefinedVar { name, orig_line } => {
                format!("Redefined variable {name}. Originally defined on line {orig_line}")
            }
            PreProcessErrorKind::EmptyUndef => "Empty undef".to_string(),
        };
        write!(f, "{message}")
    }
}

#[derive(Debug, Clone)]
pub struct PreProcessError {
    line: usize,
    kind: PreProcessErrorKind,
}

impl PreProcessError {
    fn new(line: usize, kind: PreProcessErrorKind) -> Self {
        Self { line, kind }
    }
}

impl Display for PreProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error on line {}: {}", self.line, self.kind)
    }
}

impl Error for PreProcessError {}

fn strip_comment(input: &str) -> &str {
    if let Some(idx) = input.find("//") {
        input.get(..idx).unwrap()
    } else {
        input
    }
}

struct PreProcessor<'a> {
    defines: HashMap<&'a str, (usize, &'a str)>,
    
}

pub fn preprocess(input: &str) -> Result<String, PreProcessError> {
    let mut defines = HashMap::new();

    let lines = input.lines().map(strip_comment);

    let mut output = vec![];

    for (i, l) in lines.enumerate() {
        let i = i + 1;
        if let Some(rest) = l.strip_prefix("#define ") {
            let trimmed = rest.trim_start();

            if trimmed.is_empty() {
                return Err(PreProcessError::new(i, PreProcessErrorKind::EmptyDefine));
            }

            let (name, contents) = trimmed.split_once(' ').unwrap_or((rest, ""));
            let orig = defines.insert(name.trim(), (i, contents.trim()));
            if let Some((i2, name)) = orig {
                return Err(PreProcessError::new(
                    i,
                    PreProcessErrorKind::RedefinedVar {
                        name: name.to_string(),
                        orig_line: i2,
                    },
                ));
            }
        } else if let Some(rest) = l.strip_prefix("#undef ") {
            if rest.trim().is_empty() {
                return Err(PreProcessError::new(i, PreProcessErrorKind::EmptyUndef));
            }
        } else {
            let mut replaced = l.to_string();
            for (before, (_, after)) in defines.iter() {
                replaced = replaced.replace(before, after)
            }
            if !replaced.is_empty() {
                output.push(replaced);
            }
        }
    }

    Ok(output.join("\n"))
}
