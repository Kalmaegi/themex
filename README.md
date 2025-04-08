
A command-line tool for converting VSCode themes to Neovim themes.

## Features

- Convert VSCode theme JSON files to Neovim-compatible themes
- Support for both Lua and VimL output formats
- Treesitter syntax highlighting support
- LSP semantic highlighting support
- Preserves original theme colors and styles

## Usage

Basic usage:

```bash
themex -i xx_path/your_theme.json

cargo run -- -i .\tests\data\test_catppuccin.json
```



This will convert the VSCode theme to a Neovim theme using default settings and generate output files in the current directory.

### Command Line Arguments

- `-i, --input <PATH>`: Path to the input VSCode theme JSON file (required)
- `-f, --format <FORMAT>`: Output format, options: `vim`, `lua`, `both` (default: `both`)
- `-n, --name <NAME>`: Custom theme name
- `--treesitter`: Include Treesitter highlighting (default: `true`)
- `--lsp`: Include LSP highlighting (default: `true`)
- `-v, --verbose`: Enable verbose logging

### Examples

Generate a theme in Lua format:

```bash
themex -i your_theme.json -f lua
```

Generate a theme with a custom name in VimL format:

```bash
themex -i your_theme.json -f vim -n "my_dracula"
```


## Todo

support more and more plugin e.g. tree-sitter, telescope



## Contributing

Issues and pull requests are welcome! Since there are many keys to test, there is still some work left to be completed. If you find that certain conversions are missing or incorrect after using it, feel free to raise an issue.

## Acknowledgements

Special thanks to the creators of the following themes for their inspiration and testing resources:

- [Catppuccin](https://github.com/catppuccin) - A soothing pastel theme for various applications.
- [Eva Theme](https://github.com/fisheva/Eva-Theme) - A comfortable and semantic theme.


## License

[MIT](LICENSE) 
