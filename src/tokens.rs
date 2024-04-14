use serde::Deserialize;
use std::collections::HashMap;

pub fn parse() {
    let tokens = read_json();
    let x = tokens.get(&"$01".to_string()).unwrap();
    if let TokenOrTokenGroup::Token(a) = x {
        println!("{}", a[0].langs.get(&"en".to_string()).unwrap().accessible);
    };
}

fn read_json() -> JsonType {
serde_json::from_str(include_str!("../tokens/8X.json")).unwrap()
}

#[derive(Deserialize)]
#[serde(untagged)]
enum TokenOrTokenGroup {
    Token(Vec<Token>),
    TokenGroup(HashMap<String, Vec<Token>>),
}

type JsonType = HashMap<String, TokenOrTokenGroup>;

#[derive(Deserialize)]
struct Token {
    since: TimeStamp,
    until: Option<TimeStamp>,
    langs: HashMap<String, Language>,
}

#[derive(Deserialize)]
struct TimeStamp {
    model: String,
    #[serde(rename = "os-version")]
    os_version: String,
}

#[derive(Deserialize)]
struct Language {
    #[serde(rename = "ti-ascii")]
    ti_ascii: String,
    display: String,
    accessible: String,
    variants: Option<Vec<String>>,
}
