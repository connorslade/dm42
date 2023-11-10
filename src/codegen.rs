use std::borrow::Cow;

use crate::{
    ident::FreeIdent,
    misc::OrderedMap,
    token::{Condition, Token},
};

#[derive(Debug)]
struct Function {
    name: String,
    body: Vec<String>,
    public: bool,
}

pub struct CodeGen {
    ident: FreeIdent,
    // name => instructions
    functions: OrderedMap<String, Function>,
}

impl CodeGen {
    pub fn new() -> Self {
        Self {
            ident: FreeIdent::new(),
            functions: OrderedMap::new(),
        }
    }

    pub fn used_functions(&self) -> usize {
        self.functions.len()
    }

    pub fn used_instructions(&self) -> usize {
        self.functions.values().map(|f| f.body.len()).sum::<usize>()
    }

    pub fn used_identifiers(&self) -> usize {
        self.ident.idx as usize
    }

    fn get_function(&mut self, name: &str) -> &mut Function {
        self.functions.get_mut(name).expect("Function not found")
    }

    fn new_ident(&mut self) -> String {
        self.ident.next().expect("Out of identifiers")
    }
}

pub fn generate(codegen: &mut CodeGen, tokens: Vec<Token>) -> String {
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

    _generate(codegen, tokens, "<root>".to_owned());

    let mut out = String::new();
    for function in codegen.functions.values() {
        out.push_str(&format!("LBL {}\n", function.ident()));
        for ins in &function.body {
            out.push_str(&format!("{}\n", ins));
        }
        out.push_str("RTN\n");
    }
    out
}

fn _generate(codegen: &mut CodeGen, tokens: Vec<Token>, function: String) {
    let push_ins = |codegen: &mut CodeGen, x: String| codegen.get_function(&function).body.push(x);

    for token in tokens {
        match token {
            Token::Function { name, body, .. } => {
                assert!(codegen.functions.contains_key(&name));
                _generate(codegen, body, name);
            }
            Token::FunctionCall { name } => {
                let ins = format!("XEQ {}", codegen.get_function(&name).ident());
                codegen.get_function(&function).body.push(ins);
            }
            Token::If {
                condition,
                body,
                else_body,
            } => {
                if body.is_empty() && else_body.is_empty() {
                    continue;
                }
                gen_condition(codegen, condition, function.clone());
                let end_label = codegen.new_ident();

                // Create true branch
                if !body.is_empty() {
                    let true_label = codegen.new_ident();
                    push_ins(codegen, format!("GTO {true_label}"));

                    let mut fun = Function::new_private(true_label.clone());
                    fun.ins("DROPN 2".to_owned());
                    codegen.functions.insert(true_label.clone(), fun);
                    _generate(codegen, body, true_label.clone());
                    codegen
                        .get_function(&true_label)
                        .body
                        .push(format!("GTO {end_label}"));
                } else {
                    push_ins(codegen, format!("GTO {end_label}"));
                }

                // Create false branch
                if !else_body.is_empty() {
                    let false_label = codegen.new_ident();
                    push_ins(codegen, format!("GTO {false_label}"));

                    let mut fun = Function::new_private(false_label.clone());
                    fun.ins("DROPN 2".to_owned());
                    codegen.functions.insert(false_label.clone(), fun);
                    _generate(codegen, else_body, false_label.clone());
                    codegen
                        .get_function(&false_label)
                        .body
                        .push(format!("GTO {end_label}"));
                } else {
                    push_ins(codegen, "DROPN 2".to_owned());
                }

                push_ins(codegen, format!("LBL {end_label}"));
            }
            Token::While {
                condition,
                body,
                do_while,
            } => {
                let loop_start = codegen.new_ident();
                let loop_condition = codegen.new_ident();
                if !do_while {
                    push_ins(codegen, format!("GTO {loop_condition}"));
                } else {
                    push_ins(codegen, "0".to_string());
                    push_ins(codegen, "0".to_string());
                }
                push_ins(codegen, format!("LBL {loop_start}"));
                push_ins(codegen, "DROPN 2".to_owned());
                _generate(codegen, body, function.clone());
                if !do_while {
                    push_ins(codegen, format!("LBL {loop_condition}"));
                }
                gen_condition(codegen, condition, function.clone());
                push_ins(codegen, format!("GTO {loop_start}"));
                push_ins(codegen, "DROPN 2".to_owned());
            }
            Token::Instruction(ins) => codegen.get_function(&function).body.push(ins),
        }
    }
}

fn gen_condition(codegen: &mut CodeGen, condition: Condition, function: String) {
    match condition {
        Condition::Comparison { a, b, comparison } => {
            _generate(codegen, a, function.clone());
            _generate(codegen, b, function.clone());
            codegen
                .get_function(&function)
                .body
                .push(comparison.swap_xy().instruction().to_owned());
        }
        Condition::Raw { body } => {
            _generate(codegen, body, function.clone());
        }
    };
}

impl Function {
    fn new_private(name: String) -> Self {
        Self {
            name,
            body: Vec::new(),
            public: false,
        }
    }

    fn ins(&mut self, ins: String) {
        self.body.push(ins);
    }

    fn ident(&self) -> Cow<'_, str> {
        if self.public {
            Cow::Owned(format!("\"{}\"", self.name))
        } else {
            Cow::Borrowed(&self.name)
        }
    }
}
