use crate::ast::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Interpreter {
    globals: Rc<RefCell<HashMap<String, Value>>>,
    locals: Vec<Rc<RefCell<HashMap<String, Value>>>>,
    return_value: Option<Value>,
    break_flag: bool,
    continue_flag: bool,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = HashMap::new();
        
        // Add built-in functions
        globals.insert("print".to_string(), Value::String("builtin:print".to_string()));
        globals.insert("len".to_string(), Value::String("builtin:len".to_string()));
        globals.insert("type".to_string(), Value::String("builtin:type".to_string()));
        
        Interpreter {
            globals: Rc::new(RefCell::new(globals)),
            locals: Vec::new(),
            return_value: None,
            break_flag: false,
            continue_flag: false,
        }
    }
    
    pub fn interpret(&mut self, program: &Program) -> Result<Value, String> {
        let mut last_value = Value::Void;
        
        for stmt in &program.statements {
            last_value = self.execute_stmt(stmt)?;
            if self.return_value.is_some() {
                return Ok(self.return_value.take().unwrap_or(Value::Void));
            }
        }
        
        Ok(last_value)
    }
    
    fn execute_stmt(&mut self, stmt: &Stmt) -> Result<Value, String> {
        match stmt {
            Stmt::Expression(expr) => self.evaluate_expr(expr),
            Stmt::Let { name, value } => {
                let val = self.evaluate_expr(value)?;
                self.set_variable(name.clone(), val.clone());
                Ok(Value::Void)
            }
            Stmt::Assign { target, value } => {
                let val = self.evaluate_expr(value)?;
                self.set_variable(target.clone(), val.clone());
                Ok(val)
            }
            Stmt::If { condition, then_branch, else_branch } => {
                let cond_value = self.evaluate_expr(condition)?;
                if cond_value.is_truthy() {
                    let mut result = Value::Void;
                    for s in then_branch {
                        result = self.execute_stmt(s)?;
                        if self.return_value.is_some() || self.break_flag || self.continue_flag {
                            break;
                        }
                    }
                    Ok(result)
                } else if let Some(else_stmts) = else_branch {
                    let mut result = Value::Void;
                    for s in else_stmts {
                        result = self.execute_stmt(s)?;
                        if self.return_value.is_some() || self.break_flag || self.continue_flag {
                            break;
                        }
                    }
                    Ok(result)
                } else {
                    Ok(Value::Void)
                }
            }
            Stmt::While { condition, body } => {
                let mut result = Value::Void;
                while self.evaluate_expr(condition)?.is_truthy() {
                    for s in body {
                        result = self.execute_stmt(s)?;
                        if self.return_value.is_some() || self.break_flag || self.continue_flag {
                            break;
                        }
                    }
                    if self.break_flag {
                        self.break_flag = false;
                        break;
                    }
                    if self.continue_flag {
                        self.continue_flag = false;
                        continue;
                    }
                    if self.return_value.is_some() {
                        break;
                    }
                }
                Ok(result)
            }
            Stmt::For { init, condition, increment, body } => {
                if let Some(init_stmt) = init {
                    self.execute_stmt(init_stmt)?;
                }
                
                let mut result = Value::Void;
                loop {
                    if let Some(cond) = condition {
                        if !self.evaluate_expr(cond)?.is_truthy() {
                            break;
                        }
                    }
                    
                    for s in body {
                        result = self.execute_stmt(s)?;
                        if self.return_value.is_some() || self.break_flag || self.continue_flag {
                            break;
                        }
                    }
                    
                    if self.break_flag {
                        self.break_flag = false;
                        break;
                    }
                    if self.continue_flag {
                        self.continue_flag = false;
                    } else if self.return_value.is_some() {
                        break;
                    }
                    
                    if let Some(inc) = increment {
                        self.evaluate_expr(inc)?;
                    }
                }
                Ok(result)
            }
            Stmt::FunctionDef { name, params, body } => {
                // Store function definition as a special value
                let func_def = format!("function:{}:{}", name, params.join(","));
                self.set_variable(name.clone(), Value::String(func_def));
                Ok(Value::Void)
            }
            Stmt::Return(expr) => {
                let val = if let Some(e) = expr {
                    self.evaluate_expr(e)?
                } else {
                    Value::Void
                };
                self.return_value = Some(val.clone());
                Ok(val)
            }
            Stmt::Break => {
                self.break_flag = true;
                Ok(Value::Void)
            }
            Stmt::Continue => {
                self.continue_flag = true;
                Ok(Value::Void)
            }
            Stmt::Space { name, body } => {
                let mut result = Value::Void;
                self.locals.push(Rc::new(RefCell::new(HashMap::new())));
                for s in body {
                    result = self.execute_stmt(s)?;
                }
                self.locals.pop();
                Ok(result)
            }
            Stmt::Pub { name, kind, body } => {
                let mut result = Value::Void;
                for s in body {
                    result = self.execute_stmt(s)?;
                }
                Ok(result)
            }
            Stmt::Subpub { name, compilation_type, body } => {
                let mut result = Value::Void;
                for s in body {
                    result = self.execute_stmt(s)?;
                }
                Ok(result)
            }
            Stmt::Block(stmts) => {
                let mut result = Value::Void;
                for s in stmts {
                    result = self.execute_stmt(s)?;
                    if self.return_value.is_some() || self.break_flag || self.continue_flag {
                        break;
                    }
                }
                Ok(result)
            }
        }
    }
    
    fn evaluate_expr(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::Float(f) => Ok(Value::Float(*f)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::Boolean(b) => Ok(Value::Boolean(*b)),
            Expr::Array(elements) => {
                let mut values = Vec::new();
                for elem in elements {
                    values.push(self.evaluate_expr(elem)?);
                }
                Ok(Value::Array(values))
            }
            Expr::Object(pairs) => {
                let mut map = HashMap::new();
                for (key, val_expr) in pairs {
                    map.insert(key.clone(), self.evaluate_expr(val_expr)?);
                }
                Ok(Value::Object(map))
            }
            Expr::Identifier(name) => {
                self.get_variable(name).ok_or_else(|| format!("Undefined variable: {}", name))
            }
            Expr::Binary { left, op, right } => {
                let left_val = self.evaluate_expr(left)?;
                let right_val = self.evaluate_expr(right)?;
                self.apply_binary_op(&left_val, op, &right_val)
            }
            Expr::Unary { op, expr } => {
                let val = self.evaluate_expr(expr)?;
                self.apply_unary_op(op, &val)
            }
            Expr::Call { func, args } => {
                self.evaluate_call(func, args)
            }
            Expr::Index { object, index } => {
                let obj_val = self.evaluate_expr(object)?;
                let idx_val = self.evaluate_expr(index)?;
                match (obj_val, idx_val) {
                    (Value::Array(arr), Value::Number(idx)) => {
                        let idx = idx as usize;
                        arr.get(idx).cloned().ok_or_else(|| "Index out of bounds".to_string())
                    }
                    (Value::String(s), Value::Number(idx)) => {
                        let idx = idx as usize;
                        if idx < s.len() {
                            Ok(Value::String(s.chars().nth(idx).unwrap().to_string()))
                        } else {
                            Err("Index out of bounds".to_string())
                        }
                    }
                    _ => Err("Invalid index operation".to_string()),
                }
            }
            Expr::Member { object, member } => {
                let obj_val = self.evaluate_expr(object)?;
                match obj_val {
                    Value::Object(map) => {
                        map.get(member).cloned().ok_or_else(|| format!("Member not found: {}", member))
                    }
                    _ => Err("Cannot access member on non-object".to_string()),
                }
            }
        }
    }
    
    fn apply_binary_op(&self, left: &Value, op: &BinaryOp, right: &Value) -> Result<Value, String> {
        match (left, op, right) {
            (Value::Number(a), BinaryOp::Add, Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::Number(a), BinaryOp::Subtract, Value::Number(b)) => Ok(Value::Number(a - b)),
            (Value::Number(a), BinaryOp::Multiply, Value::Number(b)) => Ok(Value::Number(a * b)),
            (Value::Number(a), BinaryOp::Divide, Value::Number(b)) => {
                if *b == 0 { Err("Division by zero".to_string()) } else { Ok(Value::Number(a / b)) }
            }
            (Value::Number(a), BinaryOp::Modulo, Value::Number(b)) => {
                if *b == 0 { Err("Modulo by zero".to_string()) } else { Ok(Value::Number(a % b)) }
            }
            (Value::String(a), BinaryOp::Add, Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::Number(a), BinaryOp::Equal, Value::Number(b)) => Ok(Value::Boolean(a == b)),
            (Value::Number(a), BinaryOp::NotEqual, Value::Number(b)) => Ok(Value::Boolean(a != b)),
            (Value::Number(a), BinaryOp::Less, Value::Number(b)) => Ok(Value::Boolean(a < b)),
            (Value::Number(a), BinaryOp::LessEqual, Value::Number(b)) => Ok(Value::Boolean(a <= b)),
            (Value::Number(a), BinaryOp::Greater, Value::Number(b)) => Ok(Value::Boolean(a > b)),
            (Value::Number(a), BinaryOp::GreaterEqual, Value::Number(b)) => Ok(Value::Boolean(a >= b)),
            (l, BinaryOp::And, r) => Ok(Value::Boolean(l.is_truthy() && r.is_truthy())),
            (l, BinaryOp::Or, r) => Ok(Value::Boolean(l.is_truthy() || r.is_truthy())),
            _ => Err(format!("Invalid binary operation: {:?} {:?} {:?}", left, op, right)),
        }
    }
    
    fn apply_unary_op(&self, op: &UnaryOp, val: &Value) -> Result<Value, String> {
        match (op, val) {
            (UnaryOp::Negate, Value::Number(n)) => Ok(Value::Number(-n)),
            (UnaryOp::Negate, Value::Float(f)) => Ok(Value::Float(-f)),
            (UnaryOp::Not, v) => Ok(Value::Boolean(!v.is_truthy())),
            _ => Err(format!("Invalid unary operation: {:?} {:?}", op, val)),
        }
    }
    
    fn evaluate_call(&mut self, func: &Expr, args: &[Expr]) -> Result<Value, String> {
        match func {
            Expr::Identifier(name) => {
                let mut arg_vals = Vec::new();
                for arg in args {
                    arg_vals.push(self.evaluate_expr(arg)?);
                }
                
                match name.as_str() {
                    "print" => {
                        let output = arg_vals.iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<_>>()
                            .join(" ");
                        println!("{}", output);
                        Ok(Value::Void)
                    }
                    "len" => {
                        if arg_vals.is_empty() {
                            return Err("len() requires 1 argument".to_string());
                        }
                        match &arg_vals[0] {
                            Value::String(s) => Ok(Value::Number(s.len() as i64)),
                            Value::Array(arr) => Ok(Value::Number(arr.len() as i64)),
                            _ => Err("len() requires string or array".to_string()),
                        }
                    }
                    "type" => {
                        if arg_vals.is_empty() {
                            return Err("type() requires 1 argument".to_string());
                        }
                        let type_name = match &arg_vals[0] {
                            Value::Number(_) => "number",
                            Value::Float(_) => "float",
                            Value::String(_) => "string",
                            Value::Boolean(_) => "boolean",
                            Value::Array(_) => "array",
                            Value::Object(_) => "object",
                            Value::Void => "void",
                        };
                        Ok(Value::String(type_name.to_string()))
                    }
                    _ => Err(format!("Unknown function: {}", name)),
                }
            }
            Expr::Member { object, member } => {
                if let Expr::Identifier(obj_name) = &**object {
                    if obj_name == "writeutil" && member == "text" {
                        if args.len() == 1 {
                            let val = self.evaluate_expr(&args[0])?;
                            print!("{}", val.to_string());
                            return Ok(Value::Void);
                        }
                    }
                }
                Err("Unknown method call".to_string())
            }
            _ => Err("Invalid function call".to_string()),
        }
    }
    
    fn get_variable(&self, name: &str) -> Option<Value> {
        for scope in self.locals.iter().rev() {
            if let Some(val) = scope.borrow().get(name) {
                return Some(val.clone());
            }
        }
        self.globals.borrow().get(name).cloned()
    }
    
    fn set_variable(&mut self, name: String, value: Value) {
        if let Some(scope) = self.locals.last() {
            scope.borrow_mut().insert(name, value);
        } else {
            self.globals.borrow_mut().insert(name, value);
        }
    }
}
