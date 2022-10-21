# Pajson 🥧 
*an emoji language from the future*


## Pajson syntax in BNF
---

```
<comment> ::= "🙈" <eol> | "🙈" <normal-text> <eol>

<statements> ::= <statements> <statement> | <statement>

<statement> ::= <assignment> | <if-statement> | <print> | <loop> | <break-loop>

<print> ::= "💬" "✋" <expr> "🤚" <eol>

<assignment> ::= <emojis> "👈" <expr> <eol>

<expr> ::= <expr> “➕” <term> | <expr> “➖” <term> | <term>

<term> ::= <term> “❎” <factor> | <term> “➗” <factor> | <factor>

<factor> ::= “✋“ <expr> “🤚” | “➖” <factor> | <integer> | <identifier>

<identifier> ::= <emojis> | <int>

<int> ::= <int> <digit> | <digit>

<if-statement> ::= "👀" <expr> <cmp> <expr> "🫳" <nl> <statements> <nl> "🫴" <eol>

<cmp> ::= "👉" "👈" | "🐊" | "🐰" | "👎🐊" | "👎🐰" | "👎👈"

<loop> ::= "🔄" "🫳" <nl> <statements> <nl> "🫴" <eol>

<break-loop> ::= "🔚" <eol>

<eol> ::= <nl> | <etx>

<nl> ::= "U+000D"

<etx> ::= "U+2403"

<digit> ::= "🕛" | "🕐" | "🕑" | "🕒" | "🕓" | "🕔" | "🕕" | "🕖" | "🕗" | "🕘"

<emojis> ::= <emojis> <emoji> | <emoji>

<emoji> ::= "😀" |"😃" |"😄" |"😁" |"😆" |"🥹" |"😅" |"😂" |"🤣" |"🥲" |"😊" |"😇" |"🙂" |"🙃" |"😉" |"😌" |"😍" |"🥰" |"😘" |"😗" |"😙" |"😚" |"😋" |"😛" |"😝" |"😜" |"🤪" |"🤨" |"🧐" |"🤓" |"😎" |"🥸" |"🤩" |"🥳" |"😏" |"😒" |"😞" |"😔" |"😟" |"😕" |"🙁" |"😣" |"😖" |"😫" |"😩" |"🥺" |"😢" |"😭" |"😤" |"😠" |"😡" |"🤬" |"🤯" |"😳" |"🥵" |"🥶" |"😱" |"😨" |"😰" |"😥" |"😓" |"🤗" |"🤔" |"🫣" |"🤭" |"🫢" |"🫡" |"🤫" |"🫠" |"🤥" |"😶" |"🫥" |"😐" |"🫤" |"😑" |"😬" |"🙄" |"😯" |"😦" |"😧" |"😮" |"😲" |"🥱" |"😴" |"🤤" |"😪" |"😵" |"🤐" |"🥴" |"🤢" |"🤮" |"🤧" |"😷" |"🤒" |"🤕" |"🤑" |"🤠" |"😈" |"👿" |"👹" |"👺" |"🤡" |"💩" |"👻" |"💀" |"👽" |"👾" |"🤖" |"🎃" |"😺" |"😸" |"😹" |"😻" |"😼" |"😽" |"🙀" |"😿" |"😾"
```
---

## How to run:
```
cargo run < src/main.txt
```