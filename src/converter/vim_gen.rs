use anyhow::Result;
use std::collections::HashMap;

use super::color;
use crate::types::{HighlightGroup, ThemeConfig};

pub struct VimGenerator<'a> {
    theme: &'a ThemeConfig,
    highlights: &'a HashMap<String, HighlightGroup>,
}

impl<'a> VimGenerator<'a> {
    pub fn new(theme: &'a ThemeConfig, highlights: &'a HashMap<String, HighlightGroup>) -> Self {
        Self { theme, highlights }
    }

    pub fn generate(&self) -> Result<String> {
        let mut content = String::new();

        content.push_str(&format!("\" {}\n", self.theme.name));
        content.push_str("\" Converted from VSCode Theme\n");
        content.push_str("\" Maintainer: Auto-generated\n\n");

        let is_dark = self.is_dark_theme();
        content.push_str(&format!(
            "set background={}\n",
            if is_dark { "dark" } else { "light" }
        ));

        content.push_str("hi clear\n");
        content.push_str("if exists(\"syntax_on\")\n");
        content.push_str("  syntax reset\n");
        content.push_str("endif\n\n");

        let theme_name = sanitize_name(&self.theme.name);
        content.push_str(&format!("let g:colors_name = \"{}\"\n\n", theme_name));

        content.push_str("\" Highlight groups\n");

        if let Some(normal) = self.highlights.get("Normal") {
            content.push_str(&self.format_highlight_group("Normal", normal));
            content.push('\n');
        }

        for (group, highlight) in self.highlights.iter() {
            if group != "Normal" {
                content.push_str(&self.format_highlight_group(group, highlight));
                content.push('\n');
            }
        }

        content.push_str("\n\" Default links\n");
        for default_link in DEFAULT_LINKS {
            if !self.highlights.contains_key(default_link.0) {
                content.push_str(&format!("hi link {} {}\n", default_link.0, default_link.1));
            }
        }

        Ok(content)
    }

    fn format_highlight_group(&self, group: &str, highlight: &HighlightGroup) -> String {
        if let Some(link) = &highlight.link {
            return format!("hi link {} {}", group, link);
        }

        let mut parts = Vec::new();

        if let Some(fg) = &highlight.fg {
            if let Ok(normalized_fg) = color::normalize_color(fg, fg) {
                parts.push(format!("guifg={}", normalized_fg));
            }
        }

        if let Some(bg) = &highlight.bg {
            if let Ok(normalized_bg) = color::normalize_color(bg, bg) {
                parts.push(format!("guibg={}", normalized_bg));
            }
        }

        if let Some(sp) = &highlight.sp {
            if let Ok(normalized_sp) = color::normalize_color(sp, sp) {
                parts.push(format!("guisp={}", normalized_sp));
            }
        }

        let mut style_parts = Vec::new();
        if highlight.bold.unwrap_or(false) {
            style_parts.push("bold");
        }
        if highlight.italic.unwrap_or(false) {
            style_parts.push("italic");
        }
        if highlight.underline.unwrap_or(false) {
            style_parts.push("underline");
        }
        if highlight.undercurl.unwrap_or(false) {
            style_parts.push("undercurl");
        }
        if highlight.strikethrough.unwrap_or(false) {
            style_parts.push("strikethrough");
        }

        let style = if style_parts.is_empty() {
            "NONE".to_string()
        } else {
            style_parts.join(",")
        };

        parts.push(format!("gui={}", style));

        format!("hi {} {}", group, parts.join(" "))
    }

    fn is_dark_theme(&self) -> bool {
        if self.theme.theme_type == "dark" {
            return true;
        }

        // 如果未指定，则尝试根据背景色判断
        if let Some(bg) = self.theme.colors.get("editor.background") {
            return color::is_dark_color(bg);
        }

        false
    }
}

fn sanitize_name(name: &str) -> String {
    name.to_lowercase()
        .replace([' ', '-', '.', ':', '/', '\\'], "_")
}

static DEFAULT_LINKS: &[(&str, &str)] = &[
    ("Conceal", "NonText"),
    ("Ignore", "NonText"),
    ("Debug", "Special"),
    ("Italic", "Normal"),
    ("StatusLineTerm", "StatusLine"),
    ("StatusLineTermNC", "StatusLineNC"),
    ("VisualNOS", "Visual"),
];
