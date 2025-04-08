use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenColor {
    pub name: Option<String>,
    pub scope: TokenScope,
    pub settings: TokenSettings,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TokenScope {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenSettings {
    pub foreground: Option<String>,
    pub background: Option<String>,
    #[serde(rename = "fontStyle")]
    pub font_style: Option<String>,
}
