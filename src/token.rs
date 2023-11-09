use std::str::FromStr;

use anyhow::bail;

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
pub enum Condition {
    Comparison {
        a: Vec<Token>,
        b: Vec<Token>,
        comparison: Comparison,
    },
    Raw {
        body: Vec<Token>,
    },
}

#[derive(Debug)]
pub enum Comparison {
    Eq,
    Ne,
    Gt,
    Lt,
    Gte,
    Lte,
}

impl FromStr for Comparison {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            "==" => Comparison::Eq,
            "!=" => Comparison::Ne,
            ">" => Comparison::Gt,
            "<" => Comparison::Lt,
            ">=" => Comparison::Gte,
            "<=" => Comparison::Lte,
            _ => bail!("Invalid comparison operator: {}", s),
        })
    }
}
