mod symbol_table;

use super::parser::ast::AstNode;

#[derive(Debug)]
pub enum IRContextError {
    `ScopeError(String),
}

pub enum AsmWrapper {
    Lit(usize, usize), // lit 0, a
    Opr(u8, usize),    // opr 0, a
    Jmp(usize, usize), // jmp 0, a
    Jpc(usize, usize), // jpc 0, a
    Cal(u8, usize),    // cal l, a
    Int(usize, usize), // int 0, a
    Lod(u8, usize),    // lod l, a
    Sto(u8, usize),    // sto l, a
}

impl std::fmt::Display for AsmWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsmWrapper::Lit(a, b) => write!(f, "lit {}, {}", a, b),
            AsmWrapper::Opr(a, b) => write!(f, "opr {}, {}", a, b),
            AsmWrapper::Jmp(a, b) => write!(f, "jmp {}, {}", a, b),
            AsmWrapper::Jpc(a, b) => write!(f, "jpc {}, {}", a, b),
            AsmWrapper::Cal(l, a) => write!(f, "cal {}, {}", l, a),
            AsmWrapper::Int(a, b) => write!(f, "int {}, {}", a, b),
            AsmWrapper::Lod(l, a) => write!(f, "lod {}, {}", l, a),
            AsmWrapper::Sto(l, a) => write!(f, "sto {}, {}", l, a),
        }
    }
}

pub struct IRContext {
    pub instructions: Vec<AsmWrapper>,
    pub verbose: bool,
    pub source_ast: Box<AstNode>,
    pub label_counter: usize,
}

impl IRContext {
    pub fn new(ast: AstNode, verbose: bool) -> Self {
        Self {
            instructions: vec![],
            source_ast: Box::new(ast),
            verbose,
            label_counter: 0,
        }
    }
    pub fn apply(&mut self, cmd: AsmWrapper) {
        self.instructions.push(cmd);
    }
    pub fn log_ir(&self) {
        for cmd in &self.instructions {
            eprintln!("{}", cmd);
        }
    }

    pub fn log(&self, msg: &str) {
        if self.verbose {
            println!("[IRContext] {}", msg);
        }
    }

    pub fn fresh_label(&mut self, prefix: &str) -> String {
        let label = format!("{}_{}", prefix, self.label_counter);
        self.label_counter += 1;
        label
    }

    pub fn process(&mut self) {
        self.log("start_process");

        self.apply(AsmWrapper::Jmp(0, 1));

        self.log_ir();

        let block = self.source_ast.as_ref();

        self.declare_program();
    }

    pub fn declare_const(&mut self, node: &AstNode) -> Result<(), IRContextError> {
        match node {
            AstNode::ConstDecl { ident, val } => {}
            _ => Err(IRContextError::ScopeError(())),
        }
    }

    pub fn declare_program(&mut self) {
        self.log("declare_program");
    }
}
