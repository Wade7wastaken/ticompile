use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TokenTree {
    token: Option<Vec<u8>>,
    children: HashMap<char, TokenTree>,
}

impl TokenTree {
    fn add(&mut self, text: String, token_id: Vec<u8>) {
        let mut current: &mut TokenTree = self;
        for c in text.chars() {
            let new = current.children.entry(c).or_insert(TokenTree {
                token: None,
                children: HashMap::new(),
            });
            current = new;
        }
        current.token = Some(token_id);
    }

    pub fn get<'a>(&self, string: &'a str) -> (Vec<u8>, &'a str) {
        let mut current: &TokenTree = self;
        let mut last_valid_token: Option<&Vec<u8>> = None;
        let mut last_valid_token_index: Option<usize> = None;

        for (i, c) in string.char_indices() {
            if let Some(next) = current.children.get(&c) {
                current = next;
            } else {
                let idx = last_valid_token_index.unwrap();
                return (
                    last_valid_token.unwrap().clone(),
                    string.get(idx+1..).unwrap(),
                );
            }
            if let Some(t) = &current.token {
                last_valid_token = Some(t);
                last_valid_token_index = Some(i);
            }
        }
        let idx = last_valid_token_index.unwrap();
        (
            last_valid_token.unwrap().clone(),
            string.get(idx+1..).unwrap(),
        )
    }
}

fn parse_token_id(token_id_str: String) -> u8 {
    u8::from_str_radix(token_id_str.strip_prefix('$').unwrap(), 16).unwrap()
}

fn add_token(tokens: Vec<Token>, token_id: Vec<u8>, root: &mut TokenTree) {
    let text = tokens
        .last()
        .unwrap()
        .langs
        .get("en")
        .unwrap()
        .accessible
        .clone();
    root.add(text, token_id);
}

pub fn process_tokens() -> TokenTree {
    let mut root = TokenTree {
        token: None,
        children: HashMap::new(),
    };

    let tokens = read_json();

    for token in tokens {
        let root_token_id = parse_token_id(token.0);
        match token.1 {
            TokenGroup::OneByte(tokens) => {
                add_token(tokens, vec![root_token_id], &mut root);
            }
            TokenGroup::TwoByte(token_list) => {
                for (second_token_id, tokens) in token_list {
                    add_token(
                        tokens,
                        vec![root_token_id, parse_token_id(second_token_id)],
                        &mut root,
                    );
                }
            }
        }
    }

    root
}

pub fn tokenize<'a>(body: &'a str, tree: &TokenTree) -> (Vec<u8>, &'a str) {
    tree.get(body)
}

fn read_json() -> TokenData {
    serde_json::from_str(include_str!("../tokens/8X.json")).unwrap()
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
