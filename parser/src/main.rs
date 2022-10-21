use core::panic;
use std::fmt::format;
use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::str::SplitAsciiWhitespace;
use std::vec;

mod read_file_emojis;
use read_file_emojis::*;

use std::collections::HashMap;

use::std::fmt;



fn main() {
    let mut tokenizer = Tokenizer::new();
    // read_file_emojis::format_emojis();
    let tokens = match tokenizer.tokenize() {
        Err(e) => panic!("{}", e),
        Ok(token_vec) => token_vec,
    };
    dbg!(&tokens);

    let mut parser = Parser::new(tokens);
    

}


struct Tokenizer {
    code: Vec<char>,
    valid_emojis: ValidEmojis,
    valid_digits: ValidDigits,
    cur_loc: Location,
    token_vec: Vec<Token>
}
impl Tokenizer {
    
    fn new() -> Tokenizer {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap(); // reads the entierty of standard input to one String
        Tokenizer {
            code: buffer.chars().collect(),
            valid_emojis: ValidEmojis::new(),
            valid_digits: ValidDigits::new(),
            cur_loc: Location::new(),
            token_vec: vec![],
        }
        
    }

    fn print_code(&self) {
        for c in &self.code {
            if c == &'\n' {
                println!("\\n");
            } else {
                println!("{}", c);
            }
        }
    }

    fn next(&mut self) {
        self.check_comment();
        if self.code.len() > 0 {

            if self.peek() == Some('\n') {
                self.cur_loc.char = 0;
                self.cur_loc.inc_line();
            } else {
                self.cur_loc.inc_char();
            }

            self.code = self.code[1..].to_vec();
            
        }
    }

    fn is_empty(&self) -> bool {
        self.code.len() == 0
    }

    fn get_next(&mut self) -> Option<char> {
        let tmp = self.peek();
        self.next();
        tmp
        
    }
    fn peek(&mut self) -> Option<char> {
        self.check_comment();
        self.check_white_space();
        if self.code.len() == 0 {
            return None;
        } else {
            return Some(self.code[0]);
        }
    }

    fn push_token(&mut self, token_type: TokenType) {
        self.token_vec.push(
            Token {
                token_type: token_type,
                location: self.cur_loc.clone(),
            }
        )
    }

    fn tokenize(&mut self) -> Result<Vec<Token>, String>{
        while ! self.is_empty() {
            match self.peek() {
                None => (), //ok, we know it is not empty
                Some('\n') => self.next(), // empty rows are allowed between statements
                _ => {
                    match self.expect_statement() {
                        Err(e) => return Err(e),
                        Ok(_) => ()
                    }
                },
            }
        }
        return Ok(self.token_vec.clone());
    }

    fn is_user_emoji(&self, c: char) -> bool {
        self.valid_emojis.user_emojis.contains(&c)
    }

    fn is_valid_digit(&self, c: char) -> bool {
        self.valid_digits.digits.contains(&c)
    }

    fn expect_statement(&mut self) -> Result<String, String> {
        if let Some(c) = self.peek() {
            if self.is_user_emoji(c) {
                self.expect_assertion()
            } else {
                match c {
                    '👀' => {
                        match self.expect_if_statement() {
                            Ok(_) => self.ok(),
                            Err(e) => return Err(e)
                        }
                    },
                    '🔄' => {
                        match self.expect_loop() {
                            Ok(_) => self.ok(),
                            Err(e) => return Err(e)
                        }
                    },
                    '💬' => {
                        match self.expect_print() {
                            Ok(_) => self.ok(),
                            Err(e) => return Err(e)
                        }
                    },
                    '🔚' => {
                        self.push_token(TokenType::BreakKeyword);
                        self.next();
                        match self.expect_eol() {
                            Ok(_) => return self.ok(),
                            Err(e) => return Err(e),
                        }
                    },
                    '\n' => {
                        self.next();
                        self.ok() // dont add this to token vec
                    }
                    _ => {
                        match self.expect_identifier() {
                            Ok(_) => {
                                println!("got here eheheh");
                                return self.expect_eol();
                            },
                            Err(e) => return Err(e)
                        }
                    }
                }
            }
        } else {
            Ok("empty is allowed".to_string())
        }

    }

    fn expect_cmp(&mut self) -> Result<String, String>{
        match self.peek() {
            Some('👎') => {
                self.next();
                match self.peek() {
                    Some('🐰') => {
                        self.push_token(TokenType::GEQ);
                        self.next();
                    },
                    Some('🐊') => {
                        self.push_token(TokenType::LEQ);
                        self.next();
                    },
                    Some('👈') => {
                        self.push_token(TokenType::NEQ);
                        self.next();
                    },
                    _ => return self.error("expected '🐰'|'🐊'|'👈'")
                }
            },
            Some('🐰') => {
                self.push_token(TokenType::LT);
                self.next();
            },
            Some('🐊') => {
                self.push_token(TokenType::GT);
                self.next();
            },
            Some('👉') => {
                self.push_token(TokenType::EQ);
                self.next();
                match self.expect('👈') {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
            },
            _ => return self.error("expected cmp")
        }

        Ok("".to_string())

    }
    fn expect_if_statement(&mut self) -> Result<String, String>{
        match self.expect('👀') {
            Ok(_) => self.push_token(TokenType::IfKeyword),
            Err(e) => return Err(e)
        }
        match self.expect_expression() {
            Ok(_) => (),
            Err(e) => return Err(e)
        }
        match self.expect_cmp() {
            Ok(_) => (),
            Err(e) => return Err(e)
        }
        match self.expect_expression() {
            Ok(_) => (),
            Err(e) => return Err(e)
        }
        match self.expect('🫳') {
            Ok(_) => self.push_token(TokenType::LBrace),
            Err(e) => return Err(e)
        }
        match self.expect_eol() {
            Ok(_) => (),
            Err(e) => return Err(e)
        }
        match self.expect_statement() {
            Ok(_) => (),
            Err(e) => return Err(e)
        }
        match self.expect('🫴') {
            Ok(_) => self.push_token(TokenType::RBrace),
            Err(e) => return Err(e)
        }
        match self.expect_eol() {
            Ok(_) => (),
            Err(e) => return Err(e)
        }
        Ok("".to_string())
    } 

    fn expect(&mut self, c: char) -> Result<char, String> {
        match self.get_next() {
            Some(cur_c) => {
                if cur_c == c {
                    return Ok(c);
                }
            }
            _ => ()
        }
        return Err(format!("expected '{}' at {}", c, self.cur_loc));
    }

    fn expect_print(&mut self) -> Result<String, String> {
        match self.expect('💬') {
            Ok(_) => self.push_token(TokenType::PrintKeyword),
            Err(e) => return Err(e)
        }
        match self.expect('✋') {
            Ok(_) => self.push_token(TokenType::LParen),
            Err(e) => return Err(e)
        }
        match self.expect_expression() {
            Ok(_) => (),
            Err(e) => return Err(e)
        }
        match self.expect('🤚') {
            Ok(_) => self.push_token(TokenType::RParen),
            Err(e) => return Err(e)
        }
        match self.expect_eol() {
            Ok(_) => (),
            Err(e) => return Err(e)
        }
        self.ok()
    }

    fn expect_loop(&mut self) -> Result<String, String>{
        match self.expect('🔄') {
            Ok(_) => self.push_token(TokenType::LoopKeyword),
            Err(e) => return Err(e),
        }
        match self.expect('🫳') {
            Ok(_) => self.push_token(TokenType::LBrace),
            Err(e) => return Err(e),
        }
        match self.expect_eol() {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        while let Some(c) = self.peek() {
            // println!("now checking for rbrace and eol");
            match c {
                '🫴' => break,
                _ => {
                    match self.expect_statement() {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    }
                }
            }
        }
        match self.expect('🫴') {
            Ok(_) => self.push_token(TokenType::RBrace),
            Err(e) => return Err(e),
        }
        match self.expect_eol() {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        self.ok()
    }

    fn expect_expression(&mut self) -> Result<String, String> {
        // return Ok("ok".to_string());
        match self.expect_term() {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        match self.peek() {
            Some('➕') => {
                self.push_token(TokenType::Add);
                self.next();
                return self.expect_expression();
            },
            Some('➖') => {
                self.push_token(TokenType::Subtract);
                self.next();
                return self.expect_expression();
            },
            _ => return Ok("ok".to_string())
        }
    }

    fn expect_term(&mut self) -> Result<String, String> {
        match self.expect_factor() {
            Ok(_) => (),
            Err(e) => return Err(e)
        }
        match self.peek() {
            Some('❎') => {
                self.push_token(TokenType::Multiply);
                self.next();
                return self.expect_factor();
            },
            Some('➗') => {
                self.push_token(TokenType::Divide);
                self.next();
                return self.expect_factor();
            },
            _ => return Ok("ok".to_string())
        }
    }

    fn expect_factor(&mut self) -> Result<String, String> {
        match self.peek() {
            Some(c) => {
                match c {
                    '✋' => {
                        self.push_token(TokenType::LParen);
                        self.next();
                        match self.expect_expression() {
                            Err(e) => return Err(e),
                            Ok(o) => {
                                if self.peek() == Some('🤚') {
                                    self.push_token(TokenType::RParen);
                                    self.next();
                                    return Ok(o);
                                } else {
                                    return Err(format!("expected '🤚' at {}", self.cur_loc));
                                }
                            }
                        }
                    },
                    '➖' => {
                        self.push_token(TokenType::Subtract);
                        self.next();
                        match self.expect_factor() {
                            Err(e) => return Err(e),
                            Ok(o) => return Ok(o),
                        }
                    },
                    _ => {
                        return self.expect_identifier()
                    }

                }
            },
            None => self.error("expected factor")
        }
    }

    fn expect_integer(&mut self) -> Result<String, String> {
        if ! vec![Some('🕛'), Some('🕐'), Some('🕑'), Some('🕒'), Some('🕓'), Some('🕔'), Some('🕕'), Some('🕖'), Some('🕗'), Some('🕘')].contains(&self.peek()) {
            return self.error("expected integer");
        }

        let mut int_symbols = vec![];
        while !self.is_empty() {
            match self.peek() {
                Some(c) => {
                    match c {
                        '🕛' | '🕐' | '🕑' | '🕒' | '🕓' | '🕔' | '🕕' | '🕖' | '🕗' | '🕘' => {
                            int_symbols.push(c);
                            self.next();
                        },
                        _ => break,
                    }
                },
                None => break,
            }
        }

        self.push_token(ValidDigits::generate_int(int_symbols));
        Ok("ok".to_string())
    }

    fn expect_emojis(&mut self) -> Result<String, String> {
        let mut name = String::new();
        match self.get_next() {
            None => return self.error("expected emojis"),
            Some(c) => {
                if self.is_user_emoji(c) {
                    name.push(c);
                } else {
                    return self.error("expected emojis");
                }
            }
        }

        while !self.is_empty() {
            match self.peek() {
                None => break,
                Some(c) => { 
                    if self.is_user_emoji(c) {
                        self.next();
                        name.push(c);
                    } else {
                        break;
                    }
                }
            }
        }
        self.push_token(TokenType::Identifier(Identifier::Emojis(name)));
        return Ok("ok".to_string());
    }

    fn expect_identifier(&mut self) -> Result<String, String> {
        if let Some(c) = self.peek() {
            if self.is_user_emoji(c) {
                return self.expect_emojis();
            } else if self.is_valid_digit(c) {
                return self.expect_integer();
            }
        }
        return self.error("expected identifier");
    }

    fn error(&self, msg: &str) -> Result<String, String> {
        Err(format!("{} at {}", msg, self.cur_loc))
    }
    fn ok(&self) -> Result<String, String> {
        Ok("Ok".to_string())
    }

    fn expect_assertion(&mut self) -> Result<String, String> {
        match self.expect_emojis() {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        match self.peek() {
            Some('👈') => {
                self.next();
                self.push_token(TokenType::Assign);
            },
            _ => {
                self.cur_loc.inc_char();
                return Err(format!("expected '👈' at {}", self.cur_loc));
            }
        }
        match self.expect_expression() {
            Err(e) => return Err(e),
            Ok(_) => (),
        }
        return self.expect_eol();
    }

    fn check_comment(&mut self) {
        if !self.is_empty() {
            if self.code[0] == '🙈' {
                self.code = self.code[1..].to_vec(); // remove monkey
                let mut comment = String::new();
                while !self.is_empty() {

                    let cur_c = self.code[0];
                    if cur_c == '\n' {
                        self.next();
                        break;
                    }
                    comment.push(cur_c);
                    self.code = self.code[1..].to_vec();

                }
                // println!("comment. \"{}\"", comment);
            }
        }
    }

    fn expect_eol(&mut self) -> Result<String, String> {
        match self.peek() {
            None => {
                self.cur_loc.char += 1;
                self.push_token(TokenType::EOF);
                Ok("end of code".to_string())
            },
            Some('\n') => {
                // self.cur_loc.char += 1;
                self.push_token(TokenType::EOL);
                self.next();
                Ok("ok".to_string())
            },
            _ => Err(format!("expected eol at {}", self.cur_loc))
        }
    }

    fn check_white_space(&mut self) {
        while !self.is_empty() {

            let cur_c = self.code[0];
            if cur_c == ' ' {
                self.code = self.code[1..].to_vec();
                self.cur_loc.inc_char();
            } else {
                break;
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Token {
    token_type: TokenType,
    location: Location,
}

#[derive(Debug, Clone)]
enum TokenType {
    Assign,
    Identifier(Identifier),
    IfKeyword,
    LoopKeyword,
    BreakKeyword,
    PrintKeyword,
    Not,
    EQ,
    LT,
    GT,
    GEQ,
    LEQ,
    NEQ,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Add,
    Subtract,
    Multiply,
    Divide,
    EOL,
    EOF,
}



#[derive(Clone, Debug)]
struct Location {
    line: u32,
    char: u32,
}

impl Location {
    fn new() -> Location {
        Location { line: 1, char: 1}
    }

    fn inc_char(&mut self) {
        self.char += 1;
    }

    fn inc_line(&mut self) {
        self.line += 1;
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}:{}>", self.line, self.char)
    }
}


#[derive(Debug, Clone)]
enum Identifier {
    Emojis(String),
    Int(i32),
}

struct ValidEmojis {
    user_emojis: Vec<char>
}

impl ValidEmojis {
    fn new() -> ValidEmojis {
        ValidEmojis {
            user_emojis: vec![
                '😀',
                '😃',
                '😄',
                '😁',
                '😆',
                '🥹',
                '😅',
                '😂',
                '🤣',
                '🥲',
                '😊',
                '😇',
                '🙂',
                '🙃',
                '😉',
                '😌',
                '😍',
                '🥰',
                '😘',
                '😗',
                '😙',
                '😚',
                '😋',
                '😛',
                '😝',
                '😜',
                '🤪',
                '🤨',
                '🧐',
                '🤓',
                '😎',
                '🥸',
                '🤩',
                '🥳',
                '😏',
                '😒',
                '😞',
                '😔',
                '😟',
                '😕',
                '🙁',
                '😣',
                '😖',
                '😫',
                '😩',
                '🥺',
                '😢',
                '😭',
                '😤',
                '😠',
                '😡',
                '🤬',
                '🤯',
                '😳',
                '🥵',
                '🥶',
                '😱',
                '😨',
                '😰',
                '😥',
                '😓',
                '🤗',
                '🤔',
                '🫣',
                '🤭',
                '🫢',
                '🫡',
                '🤫',
                '🫠',
                '🤥',
                '😶',
                '🫥',
                '😐',
                '🫤',
                '😑',
                '😬',
                '🙄',
                '😯',
                '😦',
                '😧',
                '😮',
                '😲',
                '🥱',
                '😴',
                '🤤',
                '😪',
                '😵',
                '🤐',
                '🥴',
                '🤢',
                '🤮',
                '🤧',
                '😷',
                '🤒',
                '🤕',
                '🤑',
                '🤠',
                '😈',
                '👿',
                '👹',
                '👺',
                '🤡',
                '💩',
                '👻',
                '💀',
                '👽',
                '👾',
                '🤖',
                '🎃',
                '😺',
                '😸',
                '😹',
                '😻',
                '😼',
                '😽',
                '🙀',
                '😿',
                '😾',
            ]
         }
    }
    fn print_as_bnf(&self) {
        for c in &self.user_emojis {
            print!("\"{}\" |", c);
        }
        println!();
    }
}

struct ValidDigits {
    digits: Vec<char>,
}
impl ValidDigits {
    fn new() -> ValidDigits {
        ValidDigits {
            digits: vec!['🕛', '🕐', '🕑', '🕒', '🕓', '🕔', '🕕', '🕖', '🕗', '🕘']
        }
    }
    fn generate_int(mut chars: Vec<char>) -> TokenType {
        let emojis = vec!['🕛', '🕐', '🕑', '🕒', '🕓', '🕔', '🕕', '🕖', '🕗', '🕘'];
        chars.reverse();
        let mut i = 0;
        let mut sum = 0;
        for char in chars {
            let index_element = emojis
            .iter()
            .position(|&x| x == char);
            if let Some(idx) = index_element {
                let idx = idx as i32;
                sum += idx*10_i32.pow(i);
            } else {
                panic!("all symbols here should be valid");
            }
            i += 1;
        }
        return TokenType::Identifier(Identifier::Int(sum));
    }
}




struct Parser {
    token_vec: Vec<Token>,
    ast: ASTnode,
    symbol_table: HashMap<String, i32>
}
impl Parser {
    fn new(token_vec: Vec<Token>) -> Parser {
        Parser {
            token_vec: token_vec,
            ast: ASTnode::StatementSeq(vec![]),
            symbol_table: HashMap::new(),
        }
    }
    fn parse(&mut self) -> Result<ASTnode, String>{
        match self.peek() {
            None => (),
            Some(t) => {
                match t.token_type {
                    TokenType::Identifier(i) => {
                        
                    },
                    _ => {

                    }
                }
            }
        }
        Ok(self.ast.clone())
    }

    fn parse_assignment(&mut self) {
        let var_name = match self.get_next() {
            Some(Token{token_type: TokenType::Identifier(Identifier::Emojis(emojis)), location: _}) => {
                emojis
            },
            _ => panic!()
        };

        loop {
            match self.peek() {
                Some(Token{token_type: TokenType::EOL, location: _}) => {
                    break;
                },
                Some(t) => {

                },
                None => panic!()
            }
        }

    }

    fn is_empty(&self) -> bool {
        self.token_vec.len() == 0
    }

    fn peek(&self) -> Option<Token> {
        if !self.is_empty() {
            None
        } else {
            Some(self.token_vec[0].clone())
        }
    }

    fn next(&mut self) {
        if ! self.is_empty() {
            self.token_vec = self.token_vec[1..].to_vec();
        }
    }

    fn get_next(&mut self) -> Option<Token> {
        let tmp = self.peek();
        self.next();
        return tmp;
    }
}

#[derive(Debug, Clone)]
enum ASTnode {
    StatementSeq(Vec<ASTnode>),
    Node(ASTtoken)
}

#[derive(Debug, Clone)]
struct ASTtoken {
    token: Token,
    children: Vec<ASTnode>
}
