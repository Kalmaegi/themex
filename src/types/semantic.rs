use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize, Deserialize)]
pub struct SemanticTokenColors {
    #[serde(flatten)]
    pub tokens: HashMap<String, SemanticSetting>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SemanticSetting {
    pub foreground: Option<String>,
    #[serde(rename = "fontStyle")]
    pub font_style: Option<String>,
}

impl SemanticTokenColors {
    pub fn get_unique_colors(&self) -> HashSet<String> {
        let mut unique_colors = HashSet::new();
        for (_, value) in self.tokens.iter() {
            if let Some(foreground) = value.foreground.clone() {
                unique_colors.insert(foreground);
            }
        }
        unique_colors
    }
}
