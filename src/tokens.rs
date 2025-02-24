use serde::Deserialize;
use std::{collections::HashMap, error::Error, fmt::Display};

#[derive(Debug, Default)]
pub struct TokenTree {
    token: Option<Vec<u8>>,
    children: HashMap<char, TokenTree>,
}

impl TokenTree {
    fn add(&mut self, text: String, token_id: Vec<u8>) {
        let mut current: &mut TokenTree = self;
        for c in text.chars() {
            let new = current.children.entry(c).or_default();
            current = new;
        }
        current.token = Some(token_id);
    }

    pub fn get<'a>(&self, string: &'a str) -> (Vec<u8>, &'a str) {
        let mut current: &TokenTree = self;
        let mut last_valid: Option<(&Vec<u8>, usize)> = None;

        for (i, c) in string.char_indices() {
            if let Some(next) = current.children.get(&c) {
                current = next;
            } else {
                let (tok, idx) = last_valid.unwrap();
                return (tok.clone(), string.get(idx + 1..).unwrap());
            }
            if let Some(t) = &current.token {
                last_valid = Some((t, i));
            }
        }
        let (tok, idx) = last_valid.unwrap();
        (tok.clone(), string.get(idx + 1..).unwrap())
    }
}

#[derive(Debug, Clone)]
struct InvalidTokenIdStr(String);

impl Error for InvalidTokenIdStr {}

impl Display for InvalidTokenIdStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid token id string representation {}", self.0)
    }
}

fn parse_token_id(token_id_str: &str) -> Option<u8> {
    token_id_str
        .strip_prefix('$')
        .and_then(|x| u8::from_str_radix(x, 16).ok())
}

// finds the typeable text of a token. Because these were different across
// different versions of TI, there could be multiple entries for the same token.
// We just take the latest one.
fn token_text(tokens: Vec<Token>) -> Option<String> {
    Some(tokens.last()?.langs.get("en")?.accessible.clone())
}

fn add_token(tokens: Vec<Token>, token_id: Vec<u8>, root: &mut TokenTree) -> Option<()> {
    root.add(token_text(tokens)?, token_id);
    Some(())
}

pub fn load_tokens() -> Result<TokenTree, Box<dyn Error>> {
    let mut root = TokenTree::default();

    let tokens = read_json()?;

    for token in tokens {
        let root_token_id = parse_token_id(&token.0).ok_or(InvalidTokenIdStr(token.0))?;
        match token.1 {
            TokenGroup::OneByte(tokens) => {
                add_token(tokens, vec![root_token_id], &mut root);
            }
            TokenGroup::TwoByte(token_list) => {
                for (second_token_id, tokens) in token_list {
                    add_token(
                        tokens,
                        vec![
                            root_token_id,
                            parse_token_id(&second_token_id)
                                .ok_or(InvalidTokenIdStr(second_token_id))?,
                        ],
                        &mut root,
                    );
                }
            }
        }
    }

    Ok(root)
}

pub fn tokenize(mut program: &str, tokens: &TokenTree) -> Vec<u8> {
    let mut bytes = vec![];
    while !program.is_empty() {
        let (mut tokens, left) = tokens.get(program);
        bytes.append(&mut tokens);
        program = left;
    }
    bytes
}

fn read_json() -> Result<TokenData, Box<dyn Error>> {
    Ok(serde_json::from_str(include_str!("../tokens/8X.json"))?)
}

#[derive(Deserialize)]
struct Lifetime {
    // model: String,
    // #[serde(rename = "os-version")]
    // os_version: String,
}

#[derive(Deserialize)]
struct Language {
    // #[serde(rename = "ti-ascii")]
    // ti_ascii: String,
    // display: String,
    accessible: String,
    // variants: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct Token {
    // since: Lifetime,
    // until: Option<Lifetime>,
    langs: HashMap<String, Language>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum TokenGroup {
    OneByte(Vec<Token>),
    TwoByte(HashMap<String, Vec<Token>>),
}

type TokenData = HashMap<String, TokenGroup>;
