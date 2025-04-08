use anyhow::Result;
use std::collections::HashMap;

use super::color;
use crate::types::{HighlightGroup, ThemeConfig};

pub struct LuaGenerator<'a> {
    theme: &'a ThemeConfig,
    highlights: &'a HashMap<String, HighlightGroup>,
}

impl<'a> LuaGenerator<'a> {
    pub fn new(theme: &'a ThemeConfig, highlights: &'a HashMap<String, HighlightGroup>) -> Self {
        Self { theme, highlights }
    }

    pub fn generate(&self) -> Result<String> {
        println!("[generate_modular] theme: {:?}", self.theme.name);
        let mut content = String::new();

        content.push_str(&format!("-- Name: {}\n\n", self.theme.name));

        content.push_str("local M = {}\n\n");

        content.push_str("function M.setup()\n");

        content.push_str("  vim.cmd('hi clear')\n");
        content.push_str("  if vim.fn.exists('syntax_on') then\n");
        content.push_str("    vim.cmd('syntax reset')\n");
        content.push_str("  end\n");

        let is_dark = self.is_dark_theme();
        let theme_name = sanitize_name(&self.theme.name);
        content.push_str(&format!("  vim.g.colors_name = \"{}\"\n", theme_name));
        content.push_str("  vim.o.termguicolors = true\n");
        content.push_str(&format!(
            "  vim.o.background = \"{}\"\n\n",
            if is_dark { "dark" } else { "light" }
        ));

        content.push_str("  -- Define highlight groups\n");
        content.push_str("  local highlights = {\n");

        for (group, highlight) in self.highlights {
            content.push_str(&self.format_highlight_table(group, highlight));
        }

        content.push_str("  }\n\n");

        content.push_str("  -- Set highlights\n");
        content.push_str("  for group, styles in pairs(highlights) do\n");
        content.push_str("    M.highlight(group, styles)\n");
        content.push_str("  end\n");
        content.push_str("end\n\n");

        content.push_str("-- Helper function to set highlights\n");
        content.push_str("function M.highlight(group, styles)\n");
        content.push_str("  vim.api.nvim_set_hl(0, group, {\n");
        content.push_str("    fg = styles.fg,\n");
        content.push_str("    bg = styles.bg,\n");
        content.push_str("    sp = styles.sp,\n");
        content.push_str("    bold = styles.bold,\n");
        content.push_str("    italic = styles.italic,\n");
        content.push_str("    underline = styles.underline,\n");
        content.push_str("    undercurl = styles.undercurl,\n");
        content.push_str("    strikethrough = styles.strikethrough,\n");
        content.push_str("    link = styles.link,\n");
        content.push_str("  })\n");
        content.push_str("end\n\n");

        // content.push_str("-- Set terminal colors\n");
        // content.push_str("function M.set_terminal_colors()\n");
        // content.push_str("  local p = M.palette\n");
        // content.push_str("  -- Terminal colors (use your palette colors)\n");
        // content.push_str("  vim.g.terminal_color_0 = p.black or \"#000000\"\n");
        // content.push_str("  vim.g.terminal_color_1 = p.red or \"#ff0000\"\n");
        // content.push_str("  vim.g.terminal_color_2 = p.green or \"#00ff00\"\n");
        // content.push_str("  vim.g.terminal_color_3 = p.yellow or \"#ffff00\"\n");
        // content.push_str("  vim.g.terminal_color_4 = p.blue or \"#0000ff\"\n");
        // content.push_str("  vim.g.terminal_color_5 = p.purple or \"#ff00ff\"\n");
        // content.push_str("  vim.g.terminal_color_6 = p.cyan or \"#00ffff\"\n");
        // content.push_str("  vim.g.terminal_color_7 = p.white or \"#ffffff\"\n");
        // content.push_str("  vim.g.terminal_color_8 = p.dark_gray or \"#808080\"\n");
        // content.push_str("  vim.g.terminal_color_9 = p.light_red or \"#ff8080\"\n");
        // content.push_str("  vim.g.terminal_color_10 = p.light_green or \"#80ff80\"\n");
        // content.push_str("  vim.g.terminal_color_11 = p.light_yellow or \"#ffff80\"\n");
        // content.push_str("  vim.g.terminal_color_12 = p.light_blue or \"#8080ff\"\n");
        // content.push_str("  vim.g.terminal_color_13 = p.light_purple or \"#ff80ff\"\n");
        // content.push_str("  vim.g.terminal_color_14 = p.light_cyan or \"#80ffff\"\n");
        // content.push_str("  vim.g.terminal_color_15 = p.light_gray or \"#c0c0c0\"\n");
        // content.push_str("end\n\n");

        content.push_str("return M\n");

        Ok(content)
    }


    fn format_highlight_table(&self, group: &str, highlight: &HighlightGroup) -> String {
        println!("[format_highlight_table] group: {:?}", group);
        let bg_color = self.theme.colors.get("editor.background").unwrap();
        let mut parts = Vec::new();

        // consider deal link
        // if let Some(link) = &highlight.link {
        //     return format!("    {} = {{ link = \"{}\" }},\n", group, link);
        // }

        if let Some(fg) = &highlight.fg {
            if let Ok(normalized_fg) = color::normalize_color(fg, bg_color) {
                parts.push(format!("fg = \"{}\"", normalized_fg));
            }
        }

        if let Some(bg) = &highlight.bg {
            if let Ok(normalized_bg) = color::normalize_color(bg, bg_color) {
                println!(
                    "[format_highlight_table] normalized_bg: {:?}",
                    normalized_bg
                );
                parts.push(format!("bg = \"{}\"", normalized_bg));
            }
        }

        if let Some(sp) = &highlight.sp {
            if let Ok(normalized_sp) = color::normalize_color(sp, bg_color) {
                parts.push(format!("sp = \"{}\"", normalized_sp));
            }
        }

        if highlight.bold.unwrap_or(false) {
            parts.push("bold = true".to_string());
        }
        if highlight.italic.unwrap_or(false) {
            parts.push("italic = true".to_string());
        }
        if highlight.underline.unwrap_or(false) {
            parts.push("underline = true".to_string());
        }
        if highlight.undercurl.unwrap_or(false) {
            parts.push("undercurl = true".to_string());
        }
        if highlight.strikethrough.unwrap_or(false) {
            parts.push("strikethrough = true".to_string());
        }

        format!(
            "    {} = {{ {} }},\n",
            if group.contains('@') {
                format!("[\"{}\"]", group)
            } else {
                group.to_string()
            },
            parts.join(", ")
        )
    }

    fn is_dark_theme(&self) -> bool {
        if self.theme.theme_type == "dark" {
            return true;
        }

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
