use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use themex::converter::{ConversionOptions, OutputFormat, ThemeConverter};
use themex::{ThemeConfig, utils};

#[derive(Parser)]
#[command(author = "Kalmaegi", about = "A tool to convert VSCode themes to Neovim themes", long_about = None)]
struct Cli {
    #[arg(short, long)]
    input_path: PathBuf,

    #[arg(short, long, value_enum, default_value_t = OutputFormat::Both)]
    format: OutputFormat,

    #[arg(short, long)]
    name: Option<String>,

    #[arg(short = 'o', long)]
    output_path: Option<PathBuf>,

    #[arg(long, default_value_t = true)]
    treesitter: bool,

    #[arg(long, default_value_t = true)]
    lsp: bool,

    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    unsafe {
        if args.verbose {
            std::env::set_var("RUST_LOG", "debug");
        } else {
            std::env::set_var("RUST_LOG", "info");
        }
    }
    env_logger::init();

    let theme: ThemeConfig = utils::read_and_parse_theme(&args.input_path).unwrap();
    let options = ConversionOptions {
        output_format: args.format,
        custom_name: args.name,
        include_treesitter: args.treesitter,
        include_lsp: args.lsp,
    };
    let mut converter = ThemeConverter::new(theme, options);
    converter.convert()?;

    let output_dir = match args.output_path {
        Some(path) => path,
        None => PathBuf::from("."),
    };
    converter.save_to_files(&output_dir)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn test_theme_loading() -> Result<()> {
        let theme_path = PathBuf::from_str("./latte.json").expect("error create theme path");

        let theme = utils::read_and_parse_theme(&theme_path)?;
        assert_eq!(theme.name, "Catppuccin Latte");
        assert_eq!(theme.theme_type, "light");
        assert_eq!(theme.colors.len(), 529);
        assert_eq!(theme.semantic_highlighting, true);
        assert_eq!(
            theme.semantic_token_colors.as_ref().unwrap().tokens.len(),
            30
        );
        assert_eq!(theme.token_colors.len(), 176);

        let options = ConversionOptions {
            output_format: OutputFormat::Lua,
            custom_name: Some("test_output".to_string()),
            include_treesitter: true,
            include_lsp: true,
        };

        let mut converter = ThemeConverter::new(theme, options);
        let result = converter.convert()?;
        let output_dir = PathBuf::from(".");
        converter.save_to_files(&output_dir)?;

        Ok(())
    }

    #[test]
    fn test_cli_options_to_conversion_options() -> Result<()> {
        let args = Cli {
            input_path: PathBuf::from("dummy.json"),
            format: OutputFormat::Both,
            name: Some("custom_name".to_string()),
            output_path: Some(PathBuf::from("./test_output")),
            treesitter: true,
            lsp: false,
            verbose: true,
        };

        let options = ConversionOptions {
            output_format: args.format,
            custom_name: args.name,
            include_treesitter: args.treesitter,
            include_lsp: args.lsp,
        };

        assert_eq!(options.output_format, OutputFormat::Both);
        assert_eq!(options.custom_name, Some("custom_name".to_string()));
        assert_eq!(options.include_treesitter, true);
        assert_eq!(options.include_lsp, false);
        assert_eq!(args.output_path, Some(PathBuf::from("./test_output")));

        Ok(())
    }
}
