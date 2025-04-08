vscode theme format:

"name": "demo",
"type": "dark",
"colors": {
    "focusBorder": "#cba6f7",
    "foreground": "#cdd6f4", 
},
"semanticHighlighting": true,
"semanticTokenColors": {
    "variable.readonly.defaultLibrary:go": {
      "foreground": "#cba6f7"
    },
    "tomlArrayKey": {
      "foreground": "#89b4fa",
      "fontStyle": ""
    },
},
"tokenColors": [
{
    "name": "Basic text & variable names (incl. leading punctuation)",
    "scope": [
        "text",
    "source",
    "variable.other.readwrite",
    "punctuation.definition.variable"
    ],
    "settings": {
        "foreground": "#cdd6f4"
    }
},
{
    "name": "Parentheses, Brackets, Braces",
    "scope": "punctuation",
    "settings": {
        "foreground": "#9399b2",
        "fontStyle": ""
    }
},
{
    "scope": "entity.name.tag.documentation",
    "settings": {
        "foreground": "#cba6f7"
    }
]



vscode theme color format:#RGB、#RGBA、#RRGGBB、#RRGGBBAA
