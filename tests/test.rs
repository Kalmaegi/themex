use themex::converter::{ConversionOptions, OutputFormat, ThemeConverter};
use themex::utils;

use anyhow::Result;
use std::str::FromStr;
use std::{env, path::PathBuf};

fn get_test_file_path(filename: &str) -> PathBuf {
    let mut path = env::current_dir().expect("Can't get current directory");
    path.push("tests/data");
    path.push(filename);
    path
}

#[test]
fn test_theme_loading() -> Result<()> {
    let theme_path = get_test_file_path("test_catppuccin.json");

    let theme: themex::ThemeConfig = utils::read_and_parse_theme(&theme_path)?;
    assert_eq!(theme.name, "Catppuccin Mocha Test");
    assert_eq!(theme.theme_type, "dark");

    let options = ConversionOptions {
        output_format: OutputFormat::Lua,
        custom_name: Some("test_output".to_string()),
        include_treesitter: true,
        include_lsp: true,
    };

    let mut converter = ThemeConverter::new(theme, options);
    let result = converter.convert()?;
    let highlight = converter.get_highlights();

    for (group, highlight) in highlight.iter() {
        println!("group is {:?}, highlight is {:?}", group, highlight);
    }

    let normal = highlight.get("Normal").unwrap();
    assert_eq!(normal.fg, Some("#cdd6f4".to_string()));
    assert_eq!(normal.bg, Some("#1e1e2e".to_string()));

    assert_eq!(highlight.get("Search").unwrap().fg, None);
    assert_eq!(
        highlight.get("Search").unwrap().bg,
        Some("#3e5767".to_string())
    );

    assert_eq!(highlight.get("IncSearch").unwrap().fg, None);
    assert_eq!(
        highlight.get("IncSearch").unwrap().bg,
        Some("#5e3f53".to_string())
    );

    assert_eq!(highlight.get("CurSearch").unwrap().fg, None);
    assert_eq!(
        highlight.get("CurSearch").unwrap().bg,
        Some("#5e3f53".to_string())
    );

    Ok(())
}
