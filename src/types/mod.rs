pub mod semantic;
pub mod token;

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ThemeConfig {
    pub name: String,
    #[serde(rename = "type")]
    pub theme_type: String,
    pub colors: HashMap<String, String>,
    #[serde(rename = "semanticHighlighting")]
    pub semantic_highlighting: bool,
    #[serde(rename = "semanticTokenColors")]
    pub semantic_token_colors: Option<semantic::SemanticTokenColors>,
    #[serde(rename = "tokenColors")]
    pub token_colors: Vec<token::TokenColor>,
}

#[derive(Debug, Clone, Default)]
pub struct HighlightGroup {
    pub fg: Option<String>,
    pub bg: Option<String>,
    pub sp: Option<String>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underline: Option<bool>,
    pub undercurl: Option<bool>,
    pub strikethrough: Option<bool>,
    pub link: Option<String>,
}

impl fmt::Debug for ThemeConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let colors_summary = format!("[{} entries]", self.colors.len());
        let tokens_summary = format!("[{} entries]", self.token_colors.len());

        if f.alternate() {
            f.debug_struct("ThemeConfig")
                .field("name", &self.name)
                .field("theme_type", &self.theme_type)
                .field("colors", &colors_summary)
                .field("semantic_highlighting", &self.semantic_highlighting)
                .field("semantic_token_colors", &self.semantic_token_colors)
                .field("token_colors", &tokens_summary)
                .finish()
        } else {
            write!(
                f,
                "Theme({name}, {type_}, {colors} colors, {tokens} tokens)",
                name = self.name,
                type_ = self.theme_type,
                colors = self.colors.len(),
                tokens = self.token_colors.len()
            )
        }
    }
}

impl fmt::Display for ThemeConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = &self.name;
        let theme_type = &self.theme_type;
        let colors_count = self.colors.len();
        let tokens_count = self.token_colors.len();

        write!(
            f,
            "Theme「{name}」({theme_type}) - color count: {colors_count}, token count: {tokens_count}"
        )
    }
}

