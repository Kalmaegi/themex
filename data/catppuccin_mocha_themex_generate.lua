-- Name: Catppuccin Mocha

local M = {}

function M.setup()
  vim.cmd('hi clear')
  if vim.fn.exists('syntax_on') then
    vim.cmd('syntax reset')
  end
  vim.g.colors_name = "catppuccin_mocha"
  vim.o.termguicolors = true
  vim.o.background = "dark"

  -- Define highlight groups
  local highlights = {
    -- UI elements
    PmenuSbar = { bg = "#11111b" },
    StatusLine = { fg = "#cdd6f4", bg = "#11111b" },
    StatusLineNC = { fg = "#cdd6f4", bg = "#11111b" },
    MoreMsg = { fg = "#89b4fa" },
    LineNr = { fg = "#7f849c" },
    Search = { bg = "#89dceb" },
    SignColumn = { bg = "#1e1e2e" },
    Error = { fg = "#f38ba8" },
    Visual = { bg = "#585b70" },
    TabLine = { fg = "#6c7086", bg = "#181825" },
    NormalSB = { fg = "#cdd6f4", bg = "#181825" },
    Normal = { fg = "#cdd6f4", bg = "#1e1e2e" },
    Pmenu = { fg = "#cdd6f4", bg = "#181825" },
    CursorLine = { bg = "#cdd6f4" },
    TabLineFill = { bg = "#181825" },
    Cursor = { fg = "#f5e0dc" },
    TabLineSel = { fg = "#cba6f7", bg = "#1e1e2e" },
    CursorLineNr = { fg = "#cba6f7" },
    Whitespace = { fg = "#9399b2" },
    MatchParen = { fg = "#9399b2", bg = "#9399b2" },
    PmenuSel = { bg = "#313244" },
    WarningMsg = { fg = "#fab387" },
    IncSearch = { bg = "#3e5767" },
  }

  -- Set highlights
  for group, styles in pairs(highlights) do
    M.highlight(group, styles)
  end
end

-- Helper function to set highlights
function M.highlight(group, styles)
  vim.api.nvim_set_hl(0, group, {
    fg = styles.fg,
    bg = styles.bg,
    sp = styles.sp,
    bold = styles.bold,
    italic = styles.italic,
    underline = styles.underline,
    undercurl = styles.undercurl,
    strikethrough = styles.strikethrough,
    link = styles.link,
  })
end

return M
