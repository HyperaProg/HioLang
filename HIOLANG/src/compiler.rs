use crate::ast::*;

#[derive(Debug, Clone)]
pub enum BytecodeOp {
    // Stack operations
    PushNumber(i64),
    PushFloat(f64),
    PushString(String),
    PushBool(bool),
    Pop,
    
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    
    // Logic
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    Not,
    Negate,
    
    // Variables
    GetLocal(String),
    GetGlobal(String),
    SetLocal(String),
    SetGlobal(String),
    
    // Control flow
    JumpIfFalse(usize),
    Jump(usize),
    Call(String, usize), // function name, arg count
    Return,
    
    // IO
    Print,
    WriteUtil,
    
    // Array/Object
    ArrayCreate(usize),
    ObjectCreate(usize),
    Index,
    Member(String),
}

pub struct Compiler {
    bytecode: Vec<BytecodeOp>,
    functions: std::collections::HashMap<String, Vec<BytecodeOp>>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            bytecode: Vec::new(),
            functions: std::collections::HashMap::new(),
        }
    }
    
    pub fn compile(&mut self, program: &Program) -> Result<Vec<BytecodeOp>, String> {
        for stmt in &program.statements {
            self.compile_stmt(stmt)?;
        }
        Ok(self.bytecode.clone())
    }
    
    fn compile_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Expression(expr) => {
                self.compile_expr(expr)?;
                self.bytecode.push(BytecodeOp::Pop);
            }
            Stmt::Let { name, value } => {
                self.compile_expr(value)?;
                self.bytecode.push(BytecodeOp::SetLocal(name.clone()));
            }
            Stmt::Assign { target, value } => {
                self.compile_expr(value)?;
                self.bytecode.push(BytecodeOp::SetGlobal(target.clone()));
            }
            Stmt::If { condition, then_branch, else_branch } => {
                self.compile_expr(condition)?;
                let jump_if_false_idx = self.bytecode.len();
                self.bytecode.push(BytecodeOp::JumpIfFalse(0)); // placeholder
                
                for s in then_branch {
                    self.compile_stmt(s)?;
                }
                
                let jump_idx = self.bytecode.len();
                self.bytecode.push(BytecodeOp::Jump(0)); // placeholder
                
                let false_target = self.bytecode.len();
                if let BytecodeOp::JumpIfFalse(ref mut addr) = &mut self.bytecode[jump_if_false_idx] {
                    *addr = false_target;
                }
                
                if let Some(else_stmts) = else_branch {
                    for s in else_stmts {
                        self.compile_stmt(s)?;
                    }
                }
                
                let end_target = self.bytecode.len();
                if let BytecodeOp::Jump(ref mut addr) = &mut self.bytecode[jump_idx] {
                    *addr = end_target;
                }
            }
            Stmt::While { condition, body } => {
                let loop_start = self.bytecode.len();
                
                self.compile_expr(condition)?;
                let jump_if_false_idx = self.bytecode.len();
                self.bytecode.push(BytecodeOp::JumpIfFalse(0)); // placeholder
                
                for s in body {
                    self.compile_stmt(s)?;
                }
                
                self.bytecode.push(BytecodeOp::Jump(loop_start));
                
                let loop_end = self.bytecode.len();
                if let BytecodeOp::JumpIfFalse(ref mut addr) = &mut self.bytecode[jump_if_false_idx] {
                    *addr = loop_end;
                }
            }
            Stmt::For { init, condition, increment, body } => {
                if let Some(init_stmt) = init {
                    self.compile_stmt(init_stmt)?;
                }
                
                let loop_start = self.bytecode.len();
                
                if let Some(cond) = condition {
                    self.compile_expr(cond)?;
                } else {
                    self.bytecode.push(BytecodeOp::PushBool(true));
                }
                
                let jump_if_false_idx = self.bytecode.len();
                self.bytecode.push(BytecodeOp::JumpIfFalse(0)); // placeholder
                
                for s in body {
                    self.compile_stmt(s)?;
                }
                
                if let Some(inc) = increment {
                    self.compile_expr(inc)?;
                    self.bytecode.push(BytecodeOp::Pop);
                }
                
                self.bytecode.push(BytecodeOp::Jump(loop_start));
                
                let loop_end = self.bytecode.len();
                if let BytecodeOp::JumpIfFalse(ref mut addr) = &mut self.bytecode[jump_if_false_idx] {
                    *addr = loop_end;
                }
            }
            Stmt::FunctionDef { name, params, body } => {
                let mut func_bytecode = Vec::new();
                std::mem::swap(&mut self.bytecode, &mut func_bytecode);
                
                for s in body {
                    self.compile_stmt(s)?;
                }
                
                self.bytecode.push(BytecodeOp::Return);
                let func = self.bytecode.clone();
                
                std::mem::swap(&mut self.bytecode, &mut func_bytecode);
                self.functions.insert(name.clone(), func);
            }
            Stmt::Return(expr) => {
                if let Some(e) = expr {
                    self.compile_expr(e)?;
                }
                self.bytecode.push(BytecodeOp::Return);
            }
            Stmt::Break | Stmt::Continue => {
                // Handle in runtime for now
            }
            Stmt::Space { body, .. } => {
                for s in body {
                    self.compile_stmt(s)?;
                }
            }
            Stmt::Pub { body, .. } => {
                for s in body {
                    self.compile_stmt(s)?;
                }
            }
            Stmt::Subpub { body, .. } => {
                for s in body {
                    self.compile_stmt(s)?;
                }
            }
            Stmt::Block(stmts) => {
                for s in stmts {
                    self.compile_stmt(s)?;
                }
            }
        }
        Ok(())
    }
    
    fn compile_expr(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Number(n) => self.bytecode.push(BytecodeOp::PushNumber(*n)),
            Expr::Float(f) => self.bytecode.push(BytecodeOp::PushFloat(*f)),
            Expr::String(s) => self.bytecode.push(BytecodeOp::PushString(s.clone())),
            Expr::Boolean(b) => self.bytecode.push(BytecodeOp::PushBool(*b)),
            Expr::Identifier(name) => self.bytecode.push(BytecodeOp::GetGlobal(name.clone())),
            Expr::Binary { left, op, right } => {
                self.compile_expr(left)?;
                self.compile_expr(right)?;
                let bytecode_op = match op {
                    BinaryOp::Add => BytecodeOp::Add,
                    BinaryOp::Subtract => BytecodeOp::Subtract,
                    BinaryOp::Multiply => BytecodeOp::Multiply,
                    BinaryOp::Divide => BytecodeOp::Divide,
                    BinaryOp::Modulo => BytecodeOp::Modulo,
                    BinaryOp::Equal => BytecodeOp::Equal,
                    BinaryOp::NotEqual => BytecodeOp::NotEqual,
                    BinaryOp::Less => BytecodeOp::Less,
                    BinaryOp::LessEqual => BytecodeOp::LessEqual,
                    BinaryOp::Greater => BytecodeOp::Greater,
                    BinaryOp::GreaterEqual => BytecodeOp::GreaterEqual,
                    BinaryOp::And => BytecodeOp::And,
                    BinaryOp::Or => BytecodeOp::Or,
                };
                self.bytecode.push(bytecode_op);
            }
            Expr::Unary { op, expr } => {
                self.compile_expr(expr)?;
                let bytecode_op = match op {
                    UnaryOp::Negate => BytecodeOp::Negate,
                    UnaryOp::Not => BytecodeOp::Not,
                };
                self.bytecode.push(bytecode_op);
            }
            Expr::Call { func, args } => {
                for arg in args {
                    self.compile_expr(arg)?;
                }
                if let Expr::Identifier(name) = &**func {
                    self.bytecode.push(BytecodeOp::Call(name.clone(), args.len()));
                }
            }
            Expr::Array(elements) => {
                for elem in elements {
                    self.compile_expr(elem)?;
                }
                self.bytecode.push(BytecodeOp::ArrayCreate(elements.len()));
            }
            Expr::Object(pairs) => {
                for (_, val_expr) in pairs {
                    self.compile_expr(val_expr)?;
                }
                self.bytecode.push(BytecodeOp::ObjectCreate(pairs.len()));
            }
            Expr::Index { object, index } => {
                self.compile_expr(object)?;
                self.compile_expr(index)?;
                self.bytecode.push(BytecodeOp::Index);
            }
            Expr::Member { object, member } => {
                self.compile_expr(object)?;
                self.bytecode.push(BytecodeOp::Member(member.clone()));
            }
        }
        Ok(())
    }
    
    pub fn get_bytecode(&self) -> &Vec<BytecodeOp> {
        &self.bytecode
    }
    
    pub fn get_functions(&self) -> &std::collections::HashMap<String, Vec<BytecodeOp>> {
        &self.functions
    }
}
