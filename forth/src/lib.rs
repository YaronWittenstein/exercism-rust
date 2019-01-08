#![allow(unused)]
use std::cell::RefCell;
use std::collections::HashMap;
use std::default::Default;
use std::fmt;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;
pub type TokenValue = String;

type OpcodeFn = Fn(&Forth) -> ForthResult;

#[derive(Debug, Clone)]
enum TokenKind {
    NUM,
    CMD,
    DEF,
}

#[derive(Debug, Clone)]
pub struct Token {
    kind: TokenKind,
    value: TokenValue,
    definition: Option<Vec<String>>,
}

struct Lexer;
struct Expander;
struct Evaluator;

impl Token {
    fn new(kind: TokenKind, value: String, definition: Option<Vec<String>>) -> Self {
        Self {
            kind,
            value,
            definition,
        }
    }
}

pub struct Forth {
    items: RefCell<Vec<Value>>,
    opcodes: HashMap<String, Box<OpcodeFn>>,
    defs: HashMap<String, Vec<Token>>,
}

// default Opcodes Functions
fn forth_binary(f: &Forth, op: Box<Fn(Value, Value) -> Result<Value, Error>>) -> ForthResult {
    let a = f.pop()?;
    let b = f.pop()?;

    let c = op(a, b)?;
    f.push(c);

    Ok(())
}

fn forth_add(f: &Forth) -> ForthResult {
    forth_binary(f, Box::new(|x, y| Ok(x + y)))
}

fn forth_sub(f: &Forth) -> ForthResult {
    forth_binary(f, Box::new(|x, y| Ok(y - x)))
}

fn forth_mul(f: &Forth) -> ForthResult {
    forth_binary(f, Box::new(|x, y| Ok(x * y)))
}

fn forth_div(f: &Forth) -> ForthResult {
    forth_binary(
        f,
        Box::new(|x, y| {
            if x != 0 {
                Ok(y / x)
            } else {
                Err(Error::DivisionByZero)
            }
        }),
    )
}

fn forth_swap(f: &Forth) -> ForthResult {
    let a = f.pop()?;
    let b = f.pop()?;

    f.push(a);
    f.push(b);

    Ok(())
}

fn forth_dup(f: &Forth) -> ForthResult {
    let a = f.pop()?;

    f.push(a);
    f.push(a);

    Ok(())
}

fn forth_over(f: &Forth) -> ForthResult {
    let a = f.pop()?;
    let b = f.pop()?;

    f.push(b);
    f.push(a);
    f.push(b);

    Ok(())
}

fn forth_drop(f: &Forth) -> ForthResult {
    f.pop()?;
    Ok(())
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Lexer {
    fn get_tokens(input: &str) -> Result<Vec<Token>, Error> {
        let words: Vec<&str> = input.trim().split(' ').collect();

        let mut tokens = Vec::<Token>::new();
        let mut i = 0;

        while i < words.len() {
            match words[i] {
                ":" => {
                    i += 1;

                    let mut definition = Vec::<String>::new();

                    while i < words.len() {
                        match words[i] {
                            ";" => {
                                let def_name = definition[0].to_uppercase().to_string();
                                let def_opcodes =
                                    definition[1..].iter().map(|s| s.to_uppercase()).collect();

                                let token = Token {
                                    kind: TokenKind::DEF,
                                    value: def_name,
                                    definition: Some(def_opcodes),
                                };

                                tokens.push(token);

                                break;
                            }
                            w => definition.push(w.to_uppercase().to_string()),
                        }

                        i += 1;
                    }

                    if i == words.len() {
                        return Err(Error::InvalidWord);
                    }
                }
                w => {
                    let token = match w.parse::<i32>() {
                        Ok(v) => Token {
                            kind: TokenKind::NUM,
                            value: w.to_string(),
                            definition: None,
                        },
                        Err(e) => Token {
                            kind: TokenKind::CMD,
                            value: w.to_uppercase(),
                            definition: None,
                        },
                    };

                    tokens.push(token);
                }
            }

            i += 1;
        }

        Ok(tokens)
    }
}

impl Expander {
    pub fn expand(f: &mut Forth, tokens: Vec<Token>) -> Result<Vec<Token>, Error> {
        let mut expanded = Vec::<Token>::new();

        for token in tokens {
            match token.kind {
                TokenKind::NUM => expanded.push(token),
                TokenKind::CMD => match f.get_def(&token.value) {
                    None => return Err(Error::UnknownWord),
                    Some(cmd_def) => {
                        for cmd_token in cmd_def {
                            expanded.push(cmd_token.clone());
                        }
                    }
                },
                TokenKind::DEF => {
                    let def_name = token.value;
                    let mut def_tokens = Vec::<Token>::new();

                    if def_name.parse::<i32>().is_ok() {
                        return Err(Error::InvalidWord);
                    }

                    match token.definition {
                        None => panic!(
                            "data integrity: token of type `DEF` must have a definition associated"
                        ),
                        Some(def_strings) => {
                            for def_str in def_strings {
                                match def_str.parse::<i32>() {
                                    Ok(num) => {
                                        let num_token = Token {
                                            kind: TokenKind::NUM,
                                            value: def_str,
                                            definition: None,
                                        };
                                        def_tokens.push(num_token);
                                    }
                                    Err(_) => {
                                        // the token is a `CMD`, we take its definition
                                        let cmd_def = f.get_def(&def_str);

                                        match cmd_def {
                                            None => return Err(Error::UnknownWord),
                                            Some(cmd_tokens) => {
                                                // we have the expanded version of the command
                                                for cmd_token in cmd_tokens {
                                                    def_tokens.push(cmd_token.clone());
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    f.insert_def(def_name.to_uppercase(), def_tokens);
                }
            }
        }

        Ok(expanded)
    }
}

impl Evaluator {
    fn eval(f: &mut Forth, tokens: Vec<Token>) -> ForthResult {
        for token in tokens {
            match token.kind {
                TokenKind::NUM => {
                    let n = token.value.parse::<i32>().unwrap();
                    f.push(n);
                }
                TokenKind::CMD => f.execute_opcode(&token.value)?,
                TokenKind::DEF => panic!("expanded tokens should never contain `DEF` tokens"),
            }
        }

        Ok(())
    }
}

impl Forth {
    pub fn new() -> Forth {
        let mut f = Self {
            items: Default::default(),
            opcodes: Default::default(),
            defs: Default::default(),
        };

        f.insert_opcode("+".to_string(), Box::new(forth_add));
        f.insert_opcode("-".to_string(), Box::new(forth_sub));
        f.insert_opcode("*".to_string(), Box::new(forth_mul));
        f.insert_opcode("/".to_string(), Box::new(forth_div));
        f.insert_opcode("SWAP".to_string(), Box::new(forth_swap));
        f.insert_opcode("DUP".to_string(), Box::new(forth_dup));
        f.insert_opcode("OVER".to_string(), Box::new(forth_over));
        f.insert_opcode("DROP".to_string(), Box::new(forth_drop));

        let default_defs = ["+", "-", "*", "/", "SWAP", "DUP", "OVER", "DROP"];

        for def in default_defs.iter() {
            let def_token = Token {
                kind: TokenKind::CMD,
                value: def.to_string(),
                definition: None,
            };

            f.insert_def(def.to_string(), vec![def_token]);
        }

        f
    }

    pub fn stack(&self) -> Vec<Value> {
        self.items.borrow().clone()
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let origin_tokens = Lexer::get_tokens(input)?;
        let expanded_tokens = Expander::expand(self, origin_tokens)?;

        Evaluator::eval(self, expanded_tokens)?;

        Ok(())
    }

    pub fn push(&self, value: Value) {
        self.items.borrow_mut().push(value)
    }

    pub fn pop(&self) -> Result<Value, Error> {
        if self.items.borrow().len() == 0 {
            return Err(Error::StackUnderflow);
        }

        Ok(self.items.borrow_mut().pop().unwrap())
    }

    pub fn insert_def(&mut self, def_name: String, def_tokens: Vec<Token>) {
        self.defs.insert(def_name.to_uppercase(), def_tokens);
    }

    pub fn get_def(&self, def_name: &str) -> Option<&Vec<Token>> {
        self.defs.get(&def_name.to_uppercase())
    }

    pub fn insert_opcode(&mut self, opcode_name: String, opcode_fn: Box<OpcodeFn>) {
        self.opcodes.insert(opcode_name, opcode_fn);
    }

    pub fn execute_opcode(&self, name: &str) -> ForthResult {
        match self.opcodes.get(name) {
            Some(opcode_fn) => opcode_fn(self),
            None => Err(Error::UnknownWord),
        }
    }
}
