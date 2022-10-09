# Pajson 🥧 
*an emoji language from the future*


## Pajson syntax in BNF
---

```
<comment> ::= "🙈" <eol> | "🙈" <normal-text> <eol>

<statements> ::= <statements> <statement> | <statement>

<statement> ::= <assignment> | <if-statement> | <output> | <loop> | <break-loop>

<output> ::= "💬" "✋" <expr> "🤚" <eol>

<assignment> ::= <emoji> "👈" <expr> <eol>

<expr> ::= <expr> “➕” <term> | <expr> “➖” <term> | <term>

<term> ::= <term> “✖️” <factor> | <term> “➗” <factor> | <factor>

<factor> ::= “✋“ <expr> “🤚” | “➖” <factor> | <integer> | <identifier>

<identifier> ::= <emoji> | <int>

<int> ::= <int> <digit> | <digit>

<if-statement> ::= "👀" <expr> <cmp> <expr> "🫳" <nl> <statements> <nl> "🫴" <eol>

<cmp> ::= "↔️" | "🐊➡️" | "⬅️🐊" | "↔️🐊" | "🐊↔️"

<loop> ::= "🔄" "🫳" <nl> <statements> <nl> "🫴" <eol>

<break-loop> ::= "🔚" <eol>

<eol> ::= <nl> | <etx>

<nl> ::= "U+000D"

<etx> ::= "U+2403"

<digit> ::= "0️⃣" | "1️⃣" | "2️⃣" | "3️⃣" | "4️⃣" | "5️⃣" | "6️⃣" | "7️⃣" | "8️⃣" | "9️⃣"

<emoji> ::= "😀" | "😃" | "😄" | "😁" | "😆" | "🥹" | "😅" | "😂" | "🤣" | "🥲" | "😊" | "😇" | "🙂" | "🙃" | "😉" | "😌" | "😍" | "🥰" | "😘" | "😗" | "😙" | "😚" | "😋" | "😛" | "😝" | "😜" | "🤪" | "🤨" | "🧐" | "🤓" | "😎" | "🥸" | "🤩" | "🥳" | "😏" | "😒" | "😞" | "😔" | "😟" | "😕" | "🙁" | "😣" | "😖" | "😫" | "😩" | "🥺" | "😢" | "😭" | "😤" | "😠" | "😡" | "🤬" | "🤯" | "😳" | "🥵" | "🥶" | "😶‍🌫️" | "😱" | "😨" | "😰" | "😥" | "😓" | "🤗" | "🤔" | "🫣" | "🤭" | "🫢" | "🫡" | "🤫" | "🫠" | "🤥" | "😶" | "🫥" | "😐" | "🫤" | "😑" | "😬" | "🙄" | "😯" | "😦" | "😧" | "😮" | "😲" | "🥱" | "😴" | "🤤" | "😪" | "😮‍💨" | "😵" | "😵‍💫" | "🤐" | "🥴" | "🤢" | "🤮" | "🤧" | "😷" | "🤒" | "🤕" | "🤑" | "🤠" | "😈" | "👿" | "👹" | "👺" | "🤡" | "💩" | "👻" | "💀" | "☠️" | "👽" | "👾" | "🤖" | "🎃" | "😺" | "😸" | "😹" | "😻" | "😼" | "😽" | "🙀" | "😿" | "😾"
```
---