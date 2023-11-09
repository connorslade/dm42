use anyhow::{bail, Result};

#[derive(Debug)]
pub enum Token {
    Function {
        name: String,
        body: Vec<Token>,
        public: bool,
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

    buffer: String,
    tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            idx: 0,

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
            self.idx += 1;

            if self.idx >= self.chars.len() {
                break;
            }
        }
    }

    fn take(&mut self, pattern: &[u8]) -> Result<()> {
        if !self.match_ptn(pattern) {
            bail!("Expected {:?}", pattern)
        }
        Ok(())
    }

    fn match_ptn(&mut self, pattern: &[u8]) -> bool {
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
        self.idx += pattern.len();
        true
    }

    fn next_str(&mut self) -> Result<String> {
        self.buffer.clear();

        while !self.chars[self.idx].is_whitespace() {
            self.buffer.push(self.chars[self.idx]);
            self.idx += 1;
        }

        Ok(self.buffer.clone())
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
        self.skip_whitespace();
        let body = self.tokenize_function_body()?;

        self.tokens.push(Token::Function { name, body, public });
        Ok(true)
    }

    fn tokenize_function_body(&mut self) -> Result<Vec<Token>> {
        let mut depth = 1;
        while depth != 0 {
            self.idx += 1;
            match self.chars[self.idx] {
                '{' => depth += 1,
                '}' => depth -= 1,
                _ => (),
            }
        }
        self.idx += 1;
        Ok(Vec::new())
    }
}
