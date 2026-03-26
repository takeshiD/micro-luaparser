# 設計
1. `&str`
2. `Vec<{TokenKind, len}>`
3. `Vec<{SyntaxKind, len}>`
4. `GreenTree`

## Tokenizer
- LiteralとIdentifierを検知出来るようにする
- それ以外(space, plus, commentなど)は個別のトークンとして識別する
- `local`や`function`などの予約語の識別はtokenize後にSyntaxKindに変換するときに行う

```lua
---@type number
local x = 12
local M = {}

---@param x number
---@return bool
M.add = function(x) 
    return true
end
```
1. 
2. 再帰下降解析
