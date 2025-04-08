use lazy_static::lazy_static;
use std::collections::HashMap;

// 编辑器 UI 映射
pub static EDITOR_MAPPINGS: &[(&str, (&str, &str))] = &[
    // editor base colors
    ("editor.background", ("Normal", "bg")),
    ("editor.foreground", ("Normal", "fg")),
    ("editor.lineHighlightBackground", ("CursorLine", "bg")),
    ("editorCursor.foreground", ("Cursor", "fg")),
    ("editorCursor.background", ("Cursor", "bg")),
    ("editorWhitespace.foreground", ("Whitespace", "fg")),
    ("editorWhitespace.background", ("Whitespace", "bg")),
    // search colors
    // I think this is a vscode bug: https://github.com/microsoft/vscode/issues/228782
    // current search color
    ("editor.findMatchBackground", ("CurSearch", "bg")),
    ("editor.findMatchHighlightForeground", ("CurSearch", "fg")),
    ("editor.findMatchBackground", ("IncSearch", "bg")),
    ("editor.findMatchHighlightForeground", ("IncSearch", "fg")),
    // other match search color
    ("editor.findMatchHighlightBackground", ("Search", "bg")),
    ("editor.findMatchForeground", ("Search", "fg")),
    // selection colors
    ("editor.selectionBackground", ("Visual", "bg")),
    ("editor.selectionForeground", ("Visual", "fg")),
    // line number colors
    ("editorLineNumber.foreground", ("LineNr", "fg")),
    ("editorLineNumber.activeForeground", ("CursorLineNr", "fg")),
    // Neovim and VSCode don't cover exactly the same ground.
    ("editorGutter.background", ("SignColumn", "bg")),
    ("editorBracketMatch.background", ("MatchParen", "bg")),
    ("editorBracketMatch.border", ("MatchParen", "fg")),
    // ("scrollbar.shadow", ("PmenuSbar", "bg")),
    ("tab.activeBackground", ("TabLineSel", "bg")),
    ("tab.activeForeground", ("TabLineSel", "fg")),
    ("tab.inactiveBackground", ("TabLine", "bg")),
    ("tab.inactiveForeground", ("TabLine", "fg")),
    ("tab.border", ("TabLineFill", "bg")),
    ("statusBar.background", ("StatusLine", "bg")),
    ("statusBar.foreground", ("StatusLine", "fg")),
    ("statusBar.noFolderBackground", ("StatusLineNC", "bg")),
    ("statusBar.noFolderForeground", ("StatusLineNC", "fg")),
    ("activityBar.background", ("NormalSB", "bg")),
    ("activityBar.foreground", ("NormalSB", "fg")),
    ("sideBar.background", ("NormalSB", "bg")),
    ("sideBar.foreground", ("NormalSB", "fg")),
    // pop menu colors
    ("editorWidget.background", ("Pmenu", "bg")),
    ("editorWidget.foreground", ("Pmenu", "fg")),
    ("editorSuggestWidget.selectedBackground", ("PmenuSel", "bg")),
    ("editorError.foreground", ("Error", "fg")),
    ("editorWarning.foreground", ("WarningMsg", "fg")),
    ("editorInfo.foreground", ("MoreMsg", "fg")),
];

// Treesitter 映射
pub static TREESITTER_MAPPINGS: &[(&str, &[&str])] = &[
    ("Comment", &["@comment"]),
    ("Constant", &["@constant"]),
    ("String", &["@string"]),
    ("Character", &["@character"]),
    ("Number", &["@number"]),
    ("Boolean", &["@boolean"]),
    ("Float", &["@float"]),
    ("Identifier", &["@variable"]),
    ("Function", &["@function", "@method"]),
    ("Statement", &["@statement"]),
    ("Conditional", &["@conditional"]),
    ("Repeat", &["@repeat"]),
    ("Label", &["@label"]),
    ("Operator", &["@operator"]),
    ("Keyword", &["@keyword"]),
    ("Exception", &["@exception"]),
    ("PreProc", &["@preproc"]),
    ("Include", &["@include"]),
    ("Define", &["@define"]),
    ("Macro", &["@macro"]),
    ("Type", &["@type"]),
    ("StorageClass", &["@storageclass"]),
    ("Structure", &["@structure"]),
    ("Typedef", &["@typedef"]),
    ("Special", &["@special"]),
    ("SpecialChar", &["@character.special"]),
    ("Tag", &["@tag"]),
    ("Delimiter", &["@punctuation.delimiter"]),
    ("SpecialComment", &["@comment.special"]),
    ("Todo", &["@text.todo"]),
    ("Error", &["@error"]),
];

lazy_static! {
    static ref TEXTMATE_SCOPE_MAPPINGS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();

        m.insert("comment", vec!["Comment"]);
        m.insert("comment.line", vec!["Comment"]);
        m.insert("comment.block", vec!["Comment"]);
        m.insert("comment.documentation", vec!["SpecialComment"]);

        m.insert("constant", vec!["Constant"]);
        m.insert("constant.numeric", vec!["Number"]);
        m.insert("constant.character", vec!["Character"]);
        m.insert("constant.character.escape", vec!["SpecialChar"]);
        m.insert("constant.language", vec!["Constant"]);
        m.insert("constant.other", vec!["Constant"]);

        m.insert("entity.name.function", vec!["Function"]);
        m.insert("entity.name.method", vec!["Function"]);
        m.insert("entity.name.class", vec!["Type"]);
        m.insert("entity.name.struct", vec!["Structure"]);
        m.insert("entity.name.enum", vec!["Type"]);
        m.insert("entity.name.union", vec!["Structure"]);
        m.insert("entity.name.tag", vec!["Tag"]);
        m.insert("entity.other.attribute-name", vec!["Identifier"]);

        m.insert("keyword", vec!["Keyword"]);
        m.insert("keyword.control", vec!["Statement"]);
        m.insert("keyword.operator", vec!["Operator"]);

        m.insert("punctuation", vec!["Delimiter"]);
        m.insert("punctuation.definition", vec!["Delimiter"]);
        m.insert("punctuation.separator", vec!["Delimiter"]);
        m.insert("punctuation.terminator", vec!["Delimiter"]);

        m.insert("storage", vec!["StorageClass"]);
        m.insert("storage.type", vec!["Type"]);
        m.insert("storage.modifier", vec!["StorageClass"]);

        m.insert("string", vec!["String"]);

        m.insert("support.function", vec!["Function"]);
        m.insert("support.class", vec!["Type"]);
        m.insert("support.type", vec!["Type"]);
        m.insert("support.constant", vec!["Constant"]);
        m.insert("support.variable", vec!["Identifier"]);

        m.insert("variable", vec!["Identifier"]);
        m.insert("variable.parameter", vec!["Identifier"]);
        m.insert("variable.language", vec!["Identifier"]);
        m.insert("variable.other", vec!["Identifier"]);

        m.insert("markup.heading", vec!["Title"]);
        m.insert("markup.bold", vec!["Bold"]);
        m.insert("markup.italic", vec!["Italic"]);
        m.insert("markup.underline", vec!["Underlined"]);
        m.insert("markup.quote", vec!["Comment"]);
        m.insert("markup.raw", vec!["String"]);
        m.insert("markup.list", vec!["Special"]);

        m
    };

    // 从语义令牌类型到Vim高亮组的映射
    static ref SEMANTIC_TOKEN_MAPPINGS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();

        m.insert("namespace", vec!["Identifier"]);
        m.insert("type", vec!["Type"]);
        m.insert("class", vec!["Type"]);
        m.insert("enum", vec!["Type"]);
        m.insert("interface", vec!["Type"]);
        m.insert("struct", vec!["Structure"]);
        m.insert("typeParameter", vec!["Identifier"]);
        m.insert("parameter", vec!["Identifier"]);
        m.insert("variable", vec!["Identifier"]);
        m.insert("property", vec!["Identifier"]);
        m.insert("enumMember", vec!["Constant"]);
        m.insert("event", vec!["Identifier"]);
        m.insert("function", vec!["Function"]);
        m.insert("method", vec!["Function"]);
        m.insert("macro", vec!["Macro"]);
        m.insert("keyword", vec!["Keyword"]);
        m.insert("modifier", vec!["StorageClass"]);
        m.insert("comment", vec!["Comment"]);
        m.insert("string", vec!["String"]);
        m.insert("number", vec!["Number"]);
        m.insert("regexp", vec!["String"]);
        m.insert("operator", vec!["Operator"]);

        m
    };
}

pub fn get_vim_groups_for_scope(scope: &str) -> Option<Vec<&'static str>> {
    for (key, value) in TEXTMATE_SCOPE_MAPPINGS.iter() {
        if scope.contains(*key) {
            return Some(value.clone());
        }
    }
    None
}

/// 获取语义令牌类型对应的Vim高亮组
pub fn get_vim_groups_for_semantic_token(token_type: &str) -> Option<Vec<&'static str>> {
    SEMANTIC_TOKEN_MAPPINGS.get(token_type).cloned()
}
