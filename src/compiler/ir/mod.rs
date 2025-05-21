mod symbol_table;

use symbol_table::{SymbolTable, SymbolType};

use super::parser::ast::AstNode;

#[derive(Debug)]
pub enum IRContextError {
    ScopeError(String),
}
impl IRContextError {
    pub fn throw_error(str: &str) -> IRContextResult {
        return Result::Err(IRContextError::ScopeError(str.to_string()));
    }
}

#[derive(Debug, Clone)]
pub enum Asm {
    Lit(usize),     // lit 0, a
    Opr(u8, usize), // opr 0, a
    Jmp(usize),     // jmp 0, a
    Jpc(usize),     // jpc 0, a
    Cal(u8, usize), // cal l, a
    Int(usize),     // int 0, a
    Lod(u8, usize), // lod l, a
    Sto(u8, usize), // sto l, a
}
impl Asm {}

impl std::fmt::Display for Asm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Asm::Lit(a) => write!(f, "lit {}, {}", 0, a),
            Asm::Opr(a, b) => write!(f, "opr {}, {}", a, b),
            Asm::Jmp(a) => write!(f, "jmp {}, {}", 0, a),
            Asm::Jpc(a) => write!(f, "jpc {}, {}", 0, a),
            Asm::Cal(l, a) => write!(f, "cal {}, {}", l, a),
            Asm::Int(a) => write!(f, "int {}, {}", 0, a),
            Asm::Lod(l, a) => write!(f, "lod {}, {}", l, a),
            Asm::Sto(l, a) => write!(f, "sto {}, {}", l, a),
        }
    }
}

pub struct IRContext {
    pub instructions: Vec<Asm>,
    pub verbose: bool,
    pub source_ast: Option<Box<AstNode>>,
    pub label_counter: usize,
    symbol_table: SymbolTable,
}

type IRContextResult = Result<(), IRContextError>;

impl IRContext {
    pub fn new(ast: AstNode, verbose: bool) -> Self {
        Self {
            instructions: vec![],
            source_ast: Some(Box::new(ast)),
            verbose,
            label_counter: 0,
            symbol_table: SymbolTable::new(),
        }
    }
    pub fn apply_slice(&mut self, cmd: &[Asm]) {
        for instr in cmd {
            self.instructions.push(instr.clone());
        }
    }
    pub fn apply(&mut self, cmd: Asm) {
        self.instructions.push(cmd);
    }
    pub fn log_ir(&self) {
        for cmd in &self.instructions {
            eprintln!("{}", cmd);
        }
    }

    pub fn log(&self, msg: &str) {
        if self.verbose {
            eprintln!("[IRContext] {}", msg);
        }
    }

    pub fn fresh_label(&mut self, prefix: &str) -> String {
        let label = format!("{}_{}", prefix, self.label_counter);
        self.label_counter += 1;
        label
    }
    pub fn process(&mut self) {
        match self.process_handled() {
            Ok(_) => {
                self.log_ir();
                self.symbol_table.print_table();
            }
            Err(er) => {
                eprintln!("{:#?}", er);
            }
        }
    }

    pub fn process_handled(&mut self) -> IRContextResult {
        self.log("start_process");

        self.apply(Asm::Jmp(1));

        if let Some(block) = self.source_ast.take() {
            self.declare_program(&block)?;
            // можно сохранить block обратно, если нужно
            self.source_ast = Some(block);
        } else {
            panic!("No AST found");
        }
        Ok(())
    }

    pub fn declare_const(&mut self, node: &Vec<AstNode>) -> IRContextResult {
        let mut count = 0;
        for variable in node {
            if let AstNode::ConstDecl { ident, val } = variable {
                count += 1;
                self.symbol_table
                    .new_scoped(ident.clone(), SymbolType::Const, Some(*val as i32));
            }
        }

        Ok(())
    }
    pub fn declare_var(&mut self, node: &Vec<AstNode>) -> IRContextResult {
        let mut count = 0;
        for variable in node {
            if let AstNode::VarDecl { ident } = variable {
                count += 1;
                self.symbol_table
                    .new_scoped(ident.clone(), SymbolType::Const, None);
            }
        }
        self.apply(Asm::Int(3 + count));
        Ok(())
    }
    pub fn declare_assign(&mut self, lhs: &Box<AstNode>, rhs: &Box<AstNode>) -> IRContextResult {
        let item = lhs.as_ref();

        match item {
            AstNode::VariableExpr(ident) => {
                let s_symbol = self.symbol_table.get_scoped(ident);
                if s_symbol.is_none() {
                    return IRContextError::throw_error("not found");
                }
                let variable = s_symbol.unwrap();
            }
            _ => return IRContextError::throw_error("not found"),
        }
        Ok(())
    }

    pub fn declare_statement(&mut self, block: &AstNode) -> IRContextResult {
        let res = match block {
            AstNode::Block {
                name,
                consts,
                vars,
                procedures,
            } => todo!(),
            AstNode::CallDecl { ident } => todo!(),
            AstNode::PrintDecl { arg } => todo!(),
            AstNode::ReadDecl { node } => todo!(),
            AstNode::Begin(ast_nodes) => todo!(),
            AstNode::AssignStatement { lhs, rhs } => self.declare_assign(lhs, rhs),
            AstNode::While {
                condtion,
                statement,
            } => todo!(),
            AstNode::If {
                condtion,
                then_statement,
                else_statement,
            } => todo!(),
            _ => return Err(IRContextError::ScopeError("INVALID".to_string())),
        };
        Ok(())
    }

    pub fn declare_program(&mut self, block: &AstNode) -> IRContextResult {
        self.log("declare_program");
        match block {
            AstNode::Program {
                consts,
                vars,
                procedures,
                statement,
            } => {
                self.symbol_table.switch_scope("program".to_string());
                self.declare_var(vars)?;
                self.declare_const(consts)?;

                self.declare_statement(&statement)?;
                Ok(())
            }
            _ => Err(IRContextError::ScopeError("INVALID BLOCK".to_string())),
        }
    }
}
