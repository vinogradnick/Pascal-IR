use std::collections::HashMap;

pub struct SymbolTable {
    s_table: HashMap<(), String>,
}
impl SymbolTable {
    pub fn new() -> Self {
        Self {
            s_table: HashMap::new(),
        }
    }
    pub fn check_scope() {}
}
