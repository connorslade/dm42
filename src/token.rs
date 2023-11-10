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
        do_while: bool,
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

impl Comparison {
    pub fn instruction(&self) -> &str {
        match self {
            Comparison::Eq => "X=Y?",
            Comparison::Ne => "X=Y?",
            Comparison::Gt => "X>Y?",
            Comparison::Lt => "X<Y?",
            Comparison::Gte => "X≥Y?",
            Comparison::Lte => "X≤Y?",
        }
    }

    pub fn swap_xy(&self) -> Self {
        match self {
            Comparison::Eq => Comparison::Eq,
            Comparison::Ne => Comparison::Ne,
            Comparison::Gt => Comparison::Lt,
            Comparison::Lt => Comparison::Gt,
            Comparison::Gte => Comparison::Lte,
            Comparison::Lte => Comparison::Gte,
        }
    }
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
