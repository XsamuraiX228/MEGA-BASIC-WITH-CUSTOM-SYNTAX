use crate::dialect::{SyntaxDict};
use super::token::Token;
use super::token::VALID_OPERATORS;
pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    config: &'a SyntaxDict,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, config: &'a SyntaxDict) -> Self {
        Self {
            input: input,
            pos: 0,
            config
        }
    }

    fn next_token(&mut self) -> Option<Token<'a>> {
        let bytes = self.input.as_bytes();
        while self.pos < self.input.len() && bytes[self.pos] == b' '  {
            self.pos += 1
        }
        if self.pos > self.input.len() - 1 {
            return None;
        }
        let ch = bytes[self.pos] as char;

        match ch {
            '=' => {
                if self.pos + 1 < bytes.len() && bytes[self.pos + 1] == b'=' {
                    self.pos += 2;
                    return Some(Token::DoubleEqual)
                }
                self.pos += 1;
                return Some(Token::Equal);
            }
            '!' => {
                if self.pos + 1 < bytes.len() && bytes[self.pos + 1] == b'=' {
                    self.pos += 2;
                    return Some(Token::NonEqual)
                }
                self.pos += 1;
                return Some(Token::Op('!'));
            }
            '<' => {
                self.pos += 1;
                return Some(Token::Less);
            }
            '>' => {
                self.pos += 1;
                return Some(Token::Greater); 
            }
            '\r' => {
                self.pos += 1;
                if self.pos < self.input.len() && bytes[self.pos] == b'\n' {
                    self.pos += 1;
                }
                Some(Token::Newline)
            }
            '\u{fe0f}' => {
                self.pos += 1;
                if self.pos < self.input.len() && bytes[self.pos] == b'\n' {
                    self.pos += 1;
                }
                Some(Token::Newline)
            }
            '\n' => {
                self.pos += 1;
                return Some(Token::Newline);
            }
            op if VALID_OPERATORS.contains(&op) => {
                self.pos += 1;
                return Some(Token::Op(ch));
            }
            '"' => {
                self.pos += 1; 
                let start = self.pos;
                
                while self.pos < bytes.len() && bytes[self.pos] != b'"' {
                    self.pos += 1;
                }
                
                let text_str = &self.input[start..self.pos];
                self.pos += 1; 
                Some(Token::Text(text_str))
            }
            ':' => {
                self.pos += 1;
                let start = self.pos;
                while self.pos < bytes.len() {
                    if let Some(current_char) = self.input[self.pos..].chars().next() {
                        if current_char.is_whitespace() 
                            || current_char == '='
                            || current_char == '!'
                            || VALID_OPERATORS.contains(&current_char) 
                        {
                            break;
                        }
                        self.pos += current_char.len_utf8();
                    }
                }
                Some(Token::Mark(&self.input[start..self.pos]))
            }
            '0'..='9' => {
                let start = self.pos;
                while self.pos < bytes.len() && (bytes[self.pos] as char).is_ascii_digit() {
                    self.pos += 1
                }
                let num_str = &self.input[start..self.pos];
                let number = num_str.parse::<i64>().unwrap();
                return Some(Token::Number(number));
            }
            _ => {
                let start = self.pos;
                while self.pos < bytes.len() {
                    if let Some(current_char) = self.input[self.pos..].chars().next() {
                        if current_char.is_whitespace() 
                            || current_char == '='
                            || current_char == '!'
                            || VALID_OPERATORS.contains(&current_char) 
                        {
                            break;
                        }
                        self.pos += current_char.len_utf8();
                    }
                }
                let word_str = &self.input[start..self.pos];
                if let Some(kw_type) = self.config.keywords.get(word_str) {
                    return Some(Token::KeyWord(kw_type.clone()));
                }
                Some(Token::Ident(word_str))
            }
        }
    }
    pub fn tokenize(&mut self) -> Vec<Token<'a>> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        tokens
    }
}
