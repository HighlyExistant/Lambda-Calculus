use std::{collections::HashMap, process::Command};

use gelato_parser::lexer::{Lexer, tokens::{Token, Tokens}};

use crate::{app::calculator::Calculator, ast::{FromTokens, Statements}, error::ASTError};

pub mod state;
pub mod calculator;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Events {
    Nothing,
    Error,
    Exit,
    Clear,
    CreateMacro(String, String)
}

pub struct App {
    calculator: Calculator,
    macros: HashMap<String, String>
}

impl App {
    pub fn new() -> Self {
        Self { 
            calculator: Calculator::empty(),
            macros: HashMap::new(),
        }
    }
    pub fn insert_macro(&mut self, key: &str, value: &str) {
        let mut value = value.to_string();
        self.apply_macros(&mut value);
        self.macros.insert(key.to_string(), value.to_string());
    }
    // returns true if it created a valid macro
    pub fn parse_macro(&self, tokens: &mut Tokens) -> Option<(String, String)> {
        let punct = tokens.next()?;
        let ident = if punct.is_punct_subset("#") { // # will be our macro symbol
            let ident = tokens.next()?;
            ident.get_ident()?
        } else {
            return None;
        };
        if !tokens.next()?.is_punct_subset("=") {
            return None;
        }
        let statements = format!("{}", tokens).replace("# ", "#");
        Some((format!("#{}", ident.ident), statements))
    }
    pub fn process_message(&self, text: &String) -> Events {
        match text.trim() {
            "exit()" => {
                Events::Exit
            },
            "clear()" => {
                Events::Clear
            },
            _ => {
                let mut tokens = Lexer::new(text.clone()).parse();
                if let Some(a) = self.parse_macro(&mut tokens) {
                    return Events::CreateMacro(a.0, a.1);
                }
                Events::Nothing
            }
        }
    }
    pub fn clear_terminal_screen() {
        let result = if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/c", "cls"]).spawn()
        } else {
            // "clear" or "tput reset"
            Command::new("tput").arg("reset").spawn()
        };

        // Alternative solution:
        if result.is_err() {
            print!("{esc}c", esc = 27 as char);
        }
    }
    pub fn apply_macros(&mut self, text: &mut String) {
        for (from, to) in self.macros.iter() {
            *text = text.replace(from.as_str(), to.as_str());
            // println!("CHANGE {text}");
        }
    }
    pub fn run(&mut self) {
        let mut events = Events::Nothing;
        loop {
            let mut text = String::new();
            if let Err(_) = std::io::stdin().read_line(&mut text) {
                continue;
            }
            events = self.process_message(&text);
            match events {
                Events::Nothing => {}
                Events::Clear => {
                    Self::clear_terminal_screen();
                    continue
                },
                Events::Error => continue,
                Events::Exit => break,
                Events::CreateMacro(from, to) => {
                    self.insert_macro(&from, &to);
                    continue;
                }
            }
            self.apply_macros(&mut text);
            match self.calculator.input(&text) {
                Err(ASTError::EmptyTokenList) => {}
                Err(msg) => {
                    println!("{}", msg);
                    continue;
                }
                Ok(_) => {

                }
                _ => {}
            }
            while self.calculator.step().is_running() {
                println!("{}", self.calculator.statements());
                
            }
        }
    }
}