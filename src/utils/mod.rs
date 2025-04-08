use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::types::ThemeConfig;

/// 读取并解析主题文件
pub fn read_and_parse_theme(path: &Path) -> Result<ThemeConfig> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read theme file: {}", path.display()))?;

    let theme: ThemeConfig = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse theme JSON from: {}", path.display()))?;

    Ok(theme)
}

/// 确保目录存在
pub fn ensure_dir_exists(dir: &Path) -> Result<()> {
    if !dir.exists() {
        fs::create_dir_all(dir)
            .with_context(|| format!("Failed to create directory: {}", dir.display()))?;
    }
    Ok(())
}

/// 写入文件
pub fn write_file(path: &Path, content: &str) -> Result<()> {
    // 确保父目录存在
    if let Some(parent) = path.parent() {
        ensure_dir_exists(parent)?;
    }

    fs::write(path, content)
        .with_context(|| format!("Failed to write file: {}", path.display()))?;

    Ok(())
}
