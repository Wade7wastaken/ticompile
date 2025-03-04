use serde::Deserialize;
use std::{collections::HashMap, error::Error, fmt::Display};

#[derive(Debug, Default)]
pub struct TokenTrie {
    token: Option<Vec<u8>>,
    children: HashMap<char, TokenTrie>,
}

impl TokenTrie {
    pub fn load_tokens(token_json: JSONTokenData) -> Result<Self, Box<dyn Error>> {
        let mut root = Self::default();

        for token in token_json {
            let root_token_id = parse_token_id(&token.0).ok_or(InvalidTokenIdStr(token.0))?;
            match token.1 {
                JSONTokenGroup::OneByte(tokens) => {
                    root.add_token(tokens, vec![root_token_id])?;
                }
                JSONTokenGroup::TwoByte(token_list) => {
                    for (second_token_id, tokens) in token_list {
                        let token_id = vec![
                            root_token_id,
                            parse_token_id(&second_token_id)
                                .ok_or(InvalidTokenIdStr(second_token_id))?,
                        ];
                        root.add_token(tokens, token_id)?;
                    }
                }
            }
        }

        Ok(root)
    }

    fn add(&mut self, text: String, token_id: Vec<u8>) {
        let mut current: &mut TokenTrie = self;
        for c in text.chars() {
            let new = current.children.entry(c).or_default();
            current = new;
        }
        current.token = Some(token_id);
    }

    fn add_token(
        &mut self,
        tokens: Vec<JSONToken>,
        token_id: Vec<u8>,
    ) -> Result<(), TokenTextError> {
        self.add(token_text(tokens)?, token_id);
        Ok(())
    }

    fn get<'a>(&self, string: &'a str) -> (Vec<u8>, &'a str) {
        let mut current: &TokenTrie = self;
        let mut last_valid: Option<(&Vec<u8>, usize)> = None;

        let get_last_valid = |last_valid: Option<(&Vec<u8>, usize)>| {
            let (tok, idx) = last_valid.unwrap();
            (tok.clone(), string.get(idx + 1..).unwrap())
        };

        for (i, c) in string.char_indices() {
            if let Some(next) = current.children.get(&c) {
                current = next;
            } else {
                return get_last_valid(last_valid);
            }
            if let Some(t) = &current.token {
                last_valid = Some((t, i));
            }
        }
        get_last_valid(last_valid)
    }

    pub fn tokenize(&self, mut program: &str) -> Vec<u8> {
        let mut bytes = vec![];
        while !program.is_empty() {
            let (mut tokens, left) = self.get(program);
            bytes.append(&mut tokens);
            program = left;
        }
        bytes
    }
}

pub fn load_token_json() -> Result<JSONTokenData, Box<dyn Error>> {
    Ok(serde_json::from_str(include_str!("../tokens/8X.json"))?)
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

#[derive(Debug, Clone)]
enum TokenTextError {
    NoLanguages,
    NoEnglish,
}

impl Error for TokenTextError {}

impl Display for TokenTextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            TokenTextError::NoLanguages => "No languages in token",
            TokenTextError::NoEnglish => "No english language in token",
        };
        write!(f, "{text}")
    }
}

// finds the typeable text of a token. Because these were different across
// different versions of TI, there could be multiple entries for the same token.
// We just take the latest one.
fn token_text(tokens: Vec<JSONToken>) -> Result<String, TokenTextError> {
    Ok(tokens
        .last()
        .ok_or(TokenTextError::NoLanguages)?
        .langs
        .get("en")
        .ok_or(TokenTextError::NoEnglish)?
        .accessible
        .clone())
}

#[derive(Deserialize)]
pub struct JSONLifetime {
    // model: String,
    // #[serde(rename = "os-version")]
    // os_version: String,
}

#[derive(Deserialize)]
pub struct JSONLanguage {
    // #[serde(rename = "ti-ascii")]
    // ti_ascii: String,
    // display: String,
    accessible: String,
    // variants: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct JSONToken {
    // since: Lifetime,
    // until: Option<Lifetime>,
    langs: HashMap<String, JSONLanguage>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum JSONTokenGroup {
    OneByte(Vec<JSONToken>),
    TwoByte(HashMap<String, Vec<JSONToken>>),
}

pub type JSONTokenData = HashMap<String, JSONTokenGroup>;
