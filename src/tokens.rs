use itertools::Itertools;
use serde::Deserialize;
use std::{collections::HashMap, error::Error, fmt::Display, fs::File};

pub fn load_token_json() -> Result<JSONTokenData, Box<dyn Error>> {
    let file = File::open("tokens/8X.json")?;
    Ok(serde_json::from_reader(file)?)
}

// finds the typeable text of a token. Because these were different across
// different versions of TI, there could be multiple entries for the same token.
// We just take the latest one.
fn token_language(tokens: Vec<JSONToken>) -> Result<JSONLanguage, TokenTextError> {
    Ok(tokens
        .last()
        .ok_or(TokenTextError::NoLanguages)?
        .langs
        .get("en")
        .ok_or(TokenTextError::NoEnglish)?
        .clone())
}

#[derive(Debug, Clone, Default)]
pub struct TokenTrie {
    token: Option<Token>,
    children: HashMap<char, TokenTrie>,
}

impl TokenTrie {
    pub fn load_tokens(token_json: JSONTokenData) -> Result<Self, Box<dyn Error>> {
        let mut trie = Self::default();

        for (root_token_id_str, token_group) in token_json {
            let root_token_id = TokenId::new(root_token_id_str)?;

            match token_group {
                JSONTokenGroup::OneByte(token_data) => {
                    trie.add_token(token_data, root_token_id)?;
                }
                JSONTokenGroup::TwoByte(token_list) => {
                    for (second_token_id_str, token_data) in token_list {
                        let second_token_id = root_token_id.add_second(second_token_id_str)?;

                        trie.add_token(token_data, second_token_id)?;
                    }
                }
            }
        }

        Ok(trie)
    }

    fn add(&mut self, text: String, token: Token) {
        text.chars()
            .fold(self, |acc, c| acc.children.entry(c).or_default())
            .token = Some(token);
    }

    fn add_token(
        &mut self,
        token_data: Vec<JSONToken>,
        token_id: TokenId,
    ) -> Result<(), TokenTextError> {
        let lang = token_language(token_data)?;
        let new_token = Token::new(token_id, lang.display);
        for variant in lang.variants.into_iter().flatten() {
            self.add(variant, new_token.clone());
        }
        self.add(lang.accessible, new_token);
        Ok(())
    }

    fn get<'a>(&self, string: &'a str) -> (Token, &'a str) {
        let mut current: &TokenTrie = self;
        let mut last_valid: Option<(&Token, usize)> = None;

        let get_last_valid = |last_valid: Option<(&Token, usize)>| {
            let (tok, idx) = last_valid.unwrap();
            let next_char_index = string[idx..]
                .char_indices()
                .nth(1)
                .map(|(i, _)| idx + i)
                .unwrap_or(string.len()); // if there's no next char, return empty string

            (tok.clone(), string.get(next_char_index..).unwrap())
        };

        for (i, c) in string.char_indices() {
            if c == '\\' {
                if let Some(l) = last_valid {
                    return get_last_valid(Some(l));
                }
                continue;
            }
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

    pub fn tokenize(&self, mut program: &str) -> TokenList {
        let mut tokens = vec![];
        while !program.is_empty() {
            let (token, left) = self.get(program);
            tokens.push(token);
            program = left;
        }
        TokenList(tokens)
    }
}

#[derive(Debug, Clone)]
struct TokenId(Vec<u8>);

impl TokenId {
    fn new(s: String) -> Result<Self, InvalidTokenIdStr> {
        Ok(TokenId(vec![Self::parse_token_id(s)?]))
    }

    fn add_second(&self, s: String) -> Result<Self, InvalidTokenIdStr> {
        let mut cloned = self.clone();
        cloned.0.push(Self::parse_token_id(s)?);
        Ok(cloned)
    }

    fn parse_token_id(token_id_str: String) -> Result<u8, InvalidTokenIdStr> {
        token_id_str
            .strip_prefix('$')
            .and_then(|x| u8::from_str_radix(x, 16).ok())
            .ok_or(InvalidTokenIdStr(token_id_str))
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    id: TokenId,
    display: String,
}

impl Token {
    fn new(id: TokenId, display: String) -> Self {
        Token { id, display }
    }
}

#[derive(Debug, Clone)]
pub struct TokenList(Vec<Token>);

impl TokenList {
    pub fn into_bytes(self) -> Vec<u8> {
        self.0.into_iter().flat_map(|x| x.id.0).collect()
    }

    pub fn display(&self) -> String {
        self.0.iter().map(|x| &x.display).join("")
    }
}

#[derive(Debug, Clone)]
struct InvalidTokenIdStr(String);

impl Display for InvalidTokenIdStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid token id string representation {}", self.0)
    }
}

impl Error for InvalidTokenIdStr {}

#[derive(Debug, Clone)]
enum TokenTextError {
    NoLanguages,
    NoEnglish,
}

impl Display for TokenTextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            TokenTextError::NoLanguages => "No languages in token",
            TokenTextError::NoEnglish => "No english language in token",
        };
        write!(f, "{text}")
    }
}

impl Error for TokenTextError {}

#[derive(Deserialize)]
pub struct JSONLifetime {
    // model: String,
    // #[serde(rename = "os-version")]
    // os_version: String,
}

#[derive(Deserialize, Clone)]
pub struct JSONLanguage {
    // #[serde(rename = "ti-ascii")]
    // ti_ascii: String,
    display: String,
    accessible: String,
    variants: Option<Vec<String>>,
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
