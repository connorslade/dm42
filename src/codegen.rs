use std::collections::BTreeMap;

use crate::{ident::FreeIdent, token::Token};

#[derive(Debug)]
struct Function {
    name: String,
    body: Vec<String>,
    public: bool,
}

pub struct CodeGen {
    ident: FreeIdent,
    // name => instructions
    functions: BTreeMap<String, Function>,
}

impl CodeGen {
    pub fn new() -> Self {
        Self {
            ident: FreeIdent::new(),
            functions: BTreeMap::new(),
        }
    }

    fn get_function(&mut self, name: &str) -> &mut Function {
        self.functions.get_mut(name).expect("Function not found")
    }

    fn new_ident(&mut self) -> String {
        self.ident.next().expect("Out of identifiers")
    }
}

pub fn generate(tokens: Vec<Token>) -> String {
    let mut codegen = CodeGen::new();

    // Add root functions to codegen
    for func in &tokens {
        if let Token::Function { name, public, .. } = func {
            let ident = if *public {
                name.clone()
            } else {
                codegen.new_ident()
            };
            codegen.functions.insert(
                name.clone(),
                Function {
                    name: ident,
                    body: Vec::new(),
                    public: *public,
                },
            );
        }
    }
    dbg!(&codegen.functions);

    _generate(&mut codegen, tokens, "<root>".to_owned());
    dbg!(&codegen.functions);

    let mut out = String::new();
    for function in codegen.functions.values() {
        out.push_str(&function.lbl());
        for ins in &function.body {
            out.push_str(&format!("{}\n", ins));
        }
        out.push_str("RTN\n");
    }
    out
}

fn _generate(codegen: &mut CodeGen, tokens: Vec<Token>, function: String) {
    for token in tokens {
        match token {
            Token::Function { name, body, .. } => {
                assert!(codegen.functions.contains_key(&name));
                _generate(codegen, body, name);
            }
            Token::FunctionCall { name } => {
                let ident = &codegen.get_function(&name).name;
                let ins = format!("XEQ {ident}");
                codegen.get_function(&function).body.push(ins);
            }
            Token::If {
                condition,
                body,
                else_body,
            } => todo!(),
            Token::While { condition, body } => todo!(),
            Token::Instruction(ins) => codegen.get_function(&function).body.push(ins),
        }
    }
}

impl Function {
    fn lbl(&self) -> String {
        if self.public {
            format!("LBL \"{}\"\n", self.name)
        } else {
            format!("LBL {}\n", self.name)
        }
    }
}
