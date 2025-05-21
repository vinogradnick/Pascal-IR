use std::{collections::HashMap, fmt};

use crate::compiler::parser::ast::AstNode;

#[derive(Debug, Clone)]
pub enum SymbolType {
    Procedure,
    Variable,
    Const,
    Program,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    name: String,
    s_type: SymbolType,
    pub scope: String,
    pub value: Option<i32>,
}

impl fmt::Display for SymbolType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SymbolType::Procedure => "Procedure",
            SymbolType::Variable => "Variable",
            SymbolType::Const => "Const",
            SymbolType::Program => "Program",
        };
        write!(f, "{s}")
    }
}

impl Symbol {
    #[inline]
    pub fn new_const(name: String, val: i32, scope: String) -> Self {
        Self::new(name, SymbolType::Const, scope, Some(val))
    }

    pub fn new(name: String, s_type: SymbolType, scope: String, value: Option<i32>) -> Self {
        Self {
            name,
            s_type,
            scope,
            value,
        }
    }
    pub fn update_val(&mut self, new_val: i32) {
        match self.s_type {
            SymbolType::Variable => {
                self.value = Some(new_val);
            }
            _ => {
                panic!("update_val for empty Symbol");
            }
        }
    }

    pub fn print(symbols: &[&Symbol]) {
        eprintln!(
            "{:<15} {:<12} {:<20} {:<10}",
            "Name", "Type", "Scope", "Value"
        );
        eprintln!("{}", "-".repeat(55));
        for sym in symbols {
            println!(
                "{:<15} {:<12} {:<20} {:<10}",
                sym.name,
                sym.s_type,
                sym.scope,
                sym.value.map_or("None".to_string(), |v| v.to_string())
            );
        }
    }
}
pub struct SymbolTable {
    current_scope: String,

    scope_table: HashMap<String, HashMap<String, Symbol>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            current_scope: "".to_string(),
            scope_table: HashMap::new(),
        }
    }

    pub fn new_scoped(&mut self, name: String, s_type: SymbolType, value: Option<i32>) {
        self.push_raw(Symbol::new(
            name,
            s_type,
            self.current_scope.to_string(),
            value,
        ));
    }

    pub fn get_scope(&self) -> &str {
        &self.current_scope
    }

    pub fn switch_scope(&mut self, scope: String) {
        self.current_scope = scope;
    }
    pub fn get_global(&self, scope: &str, name: &str) -> Option<&Symbol> {
        self.scope_table.get(scope)?.get(name)
    }
    pub fn get_scoped(&self, name: &str) -> Option<&Symbol> {
        self.scope_table.get(&self.current_scope)?.get(name)
    }
    pub fn get_glob_list(&self) -> Vec<&Symbol> {
        self.scope_table
            .iter()
            .map(|scope| scope.1.values().collect::<Vec<_>>())
            .flatten()
            .collect::<Vec<_>>()
    }

    pub fn push(&mut self, s_type: SymbolType, node: &str, scope: String) {
        self.scope_table
            .entry(scope.to_string())
            .or_insert_with(HashMap::new)
            .insert(
                node.to_string(),
                Symbol::new(node.to_string(), s_type, scope, None),
            );
    }
    pub fn push_raw(&mut self, symbol: Symbol) {
        self.scope_table
            .entry(symbol.scope.clone())
            .or_insert_with(HashMap::new)
            .insert(symbol.name.clone(), symbol);
    }
    pub fn print_table(&self) {
        eprintln!("================SYMBOL_TABLE============================");
        eprintln!();
        Symbol::print(self.get_glob_list().as_slice());
        eprintln!();
        eprintln!("================END_SYMBOL_TABLE========================");
    }
}
