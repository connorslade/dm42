use std::{fmt::Debug, str::FromStr};

use anyhow::{bail, Context, Result};

#[derive(Debug)]
pub enum Token {
    Function {
        name: String,
        body: Vec<Token>,
        public: bool,
    },
    FunctionCall {
        name: String,
    },
    If {
        condition: Condition,
        body: Vec<Token>,
        else_body: Vec<Token>,
    },
    While {
        condition: Condition,
        body: Vec<Token>,
    },
    Instruction(String),
}

#[derive(Debug)]
pub struct Condition {
    a: Vec<Token>,
    b: Vec<Token>,
    comparison: Comparison,
}

#[derive(Debug)]
pub enum Comparison {
    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,
}

pub struct Tokenizer {
    chars: Vec<char>,
    idx: usize,
    line: usize,

    buffer: String,
    tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            idx: 0,
            line: 1,

            buffer: String::new(),
            tokens: Vec::new(),
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>> {
        while self.tokenize_function()? {}
        Ok(self.tokens)
    }

    fn skip_whitespace(&mut self) {
        if self.idx >= self.chars.len() {
            return;
        }

        while self.chars[self.idx].is_whitespace() {
            if self.chars[self.idx] == '\n' {
                self.line += 1;
            }

            self.idx += 1;
            if self.idx >= self.chars.len() {
                break;
            }
        }
    }

    fn take(&mut self, pattern: &[u8]) -> Result<()> {
        if !self.match_ptn(pattern) {
            bail!(
                "Expected `{}` on line {}",
                String::from_utf8_lossy(pattern),
                self.line
            )
        }
        Ok(())
    }

    fn peek_ptn(&self, pattern: &[u8]) -> bool {
        if self.idx + pattern.len() > self.chars.len() {
            return false;
        }

        let mut idx = 0;
        while idx < pattern.len() {
            if self.chars[self.idx + idx] != pattern[idx] as char {
                return false;
            }
            idx += 1;
        }

        true
    }

    fn match_ptn(&mut self, pattern: &[u8]) -> bool {
        let res = self.peek_ptn(pattern);
        if res {
            self.idx += pattern.len();
        }

        res
    }

    fn next_str(&mut self) -> Result<String> {
        self.buffer.clear();

        while !self.chars[self.idx].is_whitespace() {
            self.buffer.push(self.chars[self.idx]);
            self.idx += 1;
        }

        Ok(self.buffer.clone())
    }

    fn next_line(&mut self) -> Result<String> {
        self.buffer.clear();

        while !['\n', '}'].contains(&self.chars[self.idx]) {
            self.buffer.push(self.chars[self.idx]);
            self.idx += 1;
        }

        Ok(self.buffer.clone())
    }

    fn next_instruction(&mut self) -> Result<Vec<String>> {
        let line = self.next_line()?;
        Ok(line.split(',').map(|x| x.trim().to_string()).collect())
    }
}

impl Tokenizer {
    fn tokenize_function(&mut self) -> Result<bool> {
        self.skip_whitespace();
        let public = self.match_ptn(b"export");
        self.skip_whitespace();
        if self.take(b"def").is_err() {
            return Ok(false);
        }
        self.skip_whitespace();
        let name = self.next_str()?;
        let body = self.tokenize_block()?;

        self.tokens.push(Token::Function { name, body, public });
        Ok(true)
    }

    fn tokenize_block(&mut self) -> Result<Vec<Token>> {
        self.skip_whitespace();
        self.take(b"{")?;

        let mut body = Vec::new();

        loop {
            self.skip_whitespace();
            if self.peek_ptn(b"if") {
                body.push(self.tokenize_if()?);
            } else if self.peek_ptn(b"while") {
                body.push(self.tokenize_while()?);
            } else if self.match_ptn(b"}") {
                break;
            } else {
                fn map_token(x: String) -> Token {
                    if x.ends_with("()") {
                        Token::FunctionCall {
                            name: x[..x.len() - 2].to_string(),
                        }
                    } else {
                        Token::Instruction(x)
                    }
                }

                let instruction = self.next_instruction()?.into_iter().map(map_token);
                body.extend(instruction);
            }
        }

        Ok(body)
    }

    // { MAT? } == { 0 }
    fn tokenize_condition(&mut self) -> Result<Condition> {
        let a = self.tokenize_block()?;
        self.skip_whitespace();
        let comp = self
            .next_str()?
            .parse::<Comparison>()
            .with_context(|| format!("On line {}", self.line))?;
        let b = self.tokenize_block()?;

        Ok(Condition {
            a,
            b,
            comparison: comp,
        })
    }

    fn tokenize_if(&mut self) -> Result<Token> {
        self.skip_whitespace();
        self.take(b"if")?;

        let condition = self.tokenize_condition()?;
        let body = self.tokenize_block()?;

        let else_body = if self.peek_ptn(b"else") {
            self.tokenize_block()?
        } else {
            Vec::new()
        };

        Ok(Token::If {
            condition,
            body,
            else_body,
        })
    }

    fn tokenize_while(&mut self) -> Result<Token> {
        self.skip_whitespace();
        self.take(b"while")?;

        let condition = self.tokenize_condition()?;
        let body = self.tokenize_block()?;
        Ok(Token::While { condition, body })
    }
}

impl FromStr for Comparison {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            "==" => Comparison::Eq,
            "!=" => Comparison::Neq,
            ">" => Comparison::Gt,
            "<" => Comparison::Lt,
            ">=" => Comparison::Gte,
            "<=" => Comparison::Lte,
            _ => bail!("Invalid comparison operator: {}", s),
        })
    }
}

impl Debug for Tokenizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tokenizer")
            .field("idx", &self.idx)
            .field("line", &self.line)
            .field("buffer", &self.buffer)
            .field("tokens", &self.tokens)
            .field(
                "remaining",
                &self.chars[self.idx..].iter().collect::<String>(),
            )
            .finish()
    }
}
