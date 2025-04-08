use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::types::token::TokenScope;
use crate::types::{HighlightGroup, ThemeConfig};
use clap::ValueEnum;

mod color;
mod lua_gen;
mod mapping;
mod vim_gen;

pub use lua_gen::LuaGenerator;
pub use vim_gen::VimGenerator;

const DEFAULT_BG: &str = "#ffffff";
const DEFAULT_FG: &str = "#000000";
const DEFAULT_DARK_BG: &str = "#1e1e1e";
const DEFAULT_DARK_FG: &str = "#d4d4d4";

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Vim,
    Lua,
    Both,
}

#[derive(Debug, Clone)]
pub struct ConversionOptions {
    pub output_format: OutputFormat,
    pub custom_name: Option<String>,
    pub include_treesitter: bool,
    pub include_lsp: bool,
}

pub struct ThemeConverter {
    // 原始 VSCode 主题
    theme: ThemeConfig,
    // 转换选项
    options: ConversionOptions,
    // 高亮组映射 (组名 -> 高亮定义)
    highlights: HashMap<String, HighlightGroup>,
    // mappings: ThemeMappings,
    // 生成的配置内容
    vim_content: Option<String>,
    lua_content: Option<String>,
}

impl ThemeConverter {
    pub fn new(theme: ThemeConfig, options: ConversionOptions) -> Self {
        Self {
            theme,
            options,
            highlights: HashMap::new(),
            vim_content: None,
            lua_content: None,
        }
    }

    pub fn convert(&mut self) -> Result<()> {
        self.apply_basic_mappings()?;
        self.apply_editor_mappings()?;
        self.apply_syntax_mappings()?;
        self.apply_treesitter_mappings()?;
        self.generate_content()?;

        Ok(())
    }

    pub fn get_highlights(self) -> HashMap<String, HighlightGroup> {
        self.highlights.clone()
    }

    /// 应用基本映射
    fn apply_basic_mappings(&mut self) -> Result<()> {
        let is_dark = self.is_dark_theme();
        let fg = self
            .theme
            .colors
            .get("editor.foreground")
            .cloned()
            .or_else(|| Some(if is_dark { DEFAULT_DARK_FG } else { DEFAULT_FG }.to_string()));

        let bg = self
            .theme
            .colors
            .get("editor.background")
            .cloned()
            .or_else(|| Some(if is_dark { DEFAULT_DARK_BG } else { DEFAULT_BG }.to_string()));

        self.add_highlight(
            "Normal",
            HighlightGroup {
                fg,
                bg,
                ..Default::default()
            },
        );

        Ok(())
    }

    fn apply_editor_mappings(&mut self) -> Result<()> {
        mapping::EDITOR_MAPPINGS
            .iter()
            .try_for_each(|&(vs_key, (vim_group, attr))| {
                if let Some(color) = self.theme.colors.get(vs_key) {
                    self.add_highlight_attr(vim_group, attr, &color.clone());
                    Ok(())
                } else {
                    log::warn!("Missing color mapping for: {}", vs_key);
                    Ok(())
                }
            })
    }

    fn apply_syntax_mappings(&mut self) -> Result<()> {
        // 处理 tokenColors
        for token in self.theme.token_colors.iter() {
            match &token.scope {
                TokenScope::Single(scope) => {
                    println!("scope is {:?}", scope);
                    if let Some(vim_groups) = mapping::get_vim_groups_for_scope(scope) {
                        for vim_group in vim_groups {
                            let mut highlight = HighlightGroup::default();

                            if let Some(fg) = &token.settings.foreground {
                                highlight.fg = Some(fg.clone());
                            }

                            if let Some(bg) = &token.settings.background {
                                highlight.bg = Some(bg.clone());
                            }
                            self.highlights.insert(vim_group.to_string(), highlight);
                        }
                    }
                }
                TokenScope::Multiple(scopes) => {
                    for scope in scopes {
                        println!("nulti scope is {:?}", scope);
                        if let Some(vim_groups) = mapping::get_vim_groups_for_scope(scope) {
                            for vim_group in vim_groups {
                                // self.add_token_highlight(vim_group, &token.settings);
                                let mut highlight = HighlightGroup::default();

                                if let Some(fg) = &token.settings.foreground {
                                    highlight.fg = Some(fg.clone());
                                }

                                if let Some(bg) = &token.settings.background {
                                    highlight.bg = Some(bg.clone());
                                }
                                self.highlights.insert(vim_group.to_string(), highlight);
                            }
                        }
                    }
                }
            }
        }

        if let Some(token_colors) = &self.theme.semantic_token_colors {
            for (token_type, style) in &token_colors.tokens {
                if let Some(vim_groups) = mapping::get_vim_groups_for_semantic_token(token_type) {
                    for vim_group in vim_groups {
                        let mut highlight = HighlightGroup::default();

                        if let Some(fg) = &style.foreground {
                            highlight.fg = Some(fg.clone());
                        }
                        self.highlights.insert(vim_group.to_string(), highlight);
                    }
                }
            }
        }

        Ok(())
    }

    // fn apply_settings(&mut self, scope: &str, settings: &TokenSettings) {
    //     if let Some(foreground) = &settings.foreground {
    //         self.add_highlight_attr(scope, "fg", foreground);
    //     }
    //     if let Some(background) = &settings.background {
    //         self.add_highlight_attr(scope, "bg", background);
    //     }
    //     if let Some(font_style) = &settings.font_style {
    //         self.add_highlight_attr(scope, "bg", font_style);
    //     }
    // }

    fn apply_treesitter_mappings(&mut self) -> Result<()> {
        let mut new_highlights = HashMap::new(); // 临时存储需要插入的高亮组

        for (syntax_group, ts_groups) in mapping::TREESITTER_MAPPINGS.iter() {
            if let Some(highlight) = self.highlights.get(*syntax_group) {
                for ts_group in *ts_groups {
                    new_highlights.insert(ts_group.to_string(), highlight.clone());
                }
            }
        }
        self.highlights.extend(new_highlights);
        Ok(())
    }

    fn generate_content(&mut self) -> Result<()> {
        if matches!(
            self.options.output_format,
            OutputFormat::Vim | OutputFormat::Both
        ) {
            let vim_generator = VimGenerator::new(&self.theme, &self.highlights);
            self.vim_content = Some(vim_generator.generate()?);
        }

        if matches!(
            self.options.output_format,
            OutputFormat::Lua | OutputFormat::Both
        ) {
            let lua_generator = LuaGenerator::new(&self.theme, &self.highlights);
            self.lua_content = Some(lua_generator.generate()?);
        }

        Ok(())
    }

    pub fn save_to_files(&self, output_dir: &Path) -> Result<Vec<PathBuf>> {
        let mut output_files = Vec::new();
        std::fs::create_dir_all(output_dir).context("Failed to create output directory")?;

        let theme_name = self.get_theme_name();
        let sanitized_name = sanitize_name(&theme_name);

        if let Some(vim_content) = &self.vim_content {
            let vim_path = output_dir.join(format!("{}.vim", sanitized_name));
            std::fs::write(&vim_path, vim_content)
                .with_context(|| format!("Failed to write Vim file: {}", vim_path.display()))?;
            println!("save to vim_path {:?}", vim_path.clone());
            output_files.push(vim_path);
        }

        if let Some(lua_content) = &self.lua_content {
            let lua_path = output_dir.join(format!("{}.lua", sanitized_name));
            std::fs::write(&lua_path, lua_content)
                .with_context(|| format!("Failed to write Lua file: {}", lua_path.display()))?;
            println!("save to path{:?}", lua_path.clone());
            output_files.push(lua_path);
        }

        Ok(output_files)
    }

    fn is_dark_theme(&self) -> bool {
        if self.theme.theme_type == "dark" {
            return true;
        } else if self.theme.theme_type == "light" {
            return false;
        }

        if let Some(bg) = self.theme.colors.get("editor.background") {
            return color::is_dark_color(bg);
        }

        false
    }

    fn get_theme_name(&self) -> String {
        if let Some(name) = &self.options.custom_name {
            return name.clone();
        }
        self.theme.name.clone()
    }

    fn add_highlight(&mut self, group: &str, highlight: HighlightGroup) {
        self.highlights.insert(group.to_string(), highlight);
    }

    fn add_highlight_attr(&mut self, group: &str, attr: &str, value: &str) {
        let highlight = self.highlights.entry(group.to_string()).or_default();

        match attr {
            "fg" => highlight.fg = Some(value.to_string()),
            "bg" => highlight.bg = Some(value.to_string()),
            "sp" => highlight.sp = Some(value.to_string()),
            _ => {}
        }
    }

    // fn add_token_highlight(&mut self, group: &str, settings: &TokenSettings) {
    //     let mut highlight = HighlightGroup::default();

    //     if let Some(fg) = &settings.foreground {
    //         highlight.fg = Some(fg.clone());
    //     }

    //     if let Some(bg) = &settings.background {
    //         highlight.bg = Some(bg.clone());
    //     }

    //     // highlight.bold = settings.bold;
    //     // highlight.italic = settings.italic;
    //     // highlight.underline = settings.underline;

    //     self.add_highlight(group, highlight);
    // }

    // fn add_semantic_highlight(&mut self, group: &str, setting: &crate::types::semantic::SemanticSetting) {
    //     let mut highlight = HighlightGroup::default();

    //     if let Some(fg) = &setting.foreground {
    //         highlight.fg = Some(fg.clone());
    //     }

    //     // if let Some(font) = &setting.font_style {
    //     //     highlight.
    //     // }

    //     // if let Some(bg) = &setting.background {
    //     //     highlight.bg = Some(bg.clone());
    //     // }

    //     // highlight.bold = setting.bold;
    //     // highlight.italic = setting.italic;
    //     // highlight.underline = setting.underline;

    //     self.add_highlight(group, highlight);
    // }
}

fn sanitize_name(name: &str) -> String {
    name.to_lowercase()
        .replace([' ', '-', '.', ':', '/', '\\'], "_")
}
