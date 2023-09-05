use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::ast::ast::{Statement, Program, Expression, Literals, Infix, Prefix, BlockStmt, Ident};
use crate::object::env::Env;
use crate::object::object::*;

use super::builtin;

pub struct Eval {
    env: Rc<RefCell<Env>>,
    builtin: HashMap<String, Object>
}

impl Eval {
    pub fn new(env: Rc<RefCell<Env>>) -> Self {
        return Eval{
            env,
            builtin: builtin::new_builtin_functions()
        }
    }
    pub fn eval_program(&mut self, program: Program) {
        for s in program {
            let eval = self.eval_stmt(s);
            match eval {
                Object::None => {}
                Object::Error(err) => {
                    println!("Error {}", err);
                    break;
                }
                _ => {
                }
            }
        }
    }

    fn eval_block(&mut self, block: BlockStmt) -> Object{
        for s in block { 
            let evaled = self.eval_stmt(s);
            match evaled {
                Object::Return(o) => {
                    return *o
                }
                Object::Error(err) => {
                    return Object::Error(err)
                }
                _ => {
                }
            }
        }

        Object::None
    }

    fn eval_stmt(&mut self, stmt: Statement) -> Object {
        match stmt {
            Statement::ExpressionStmt(exp) => {
                self.eval_exp(exp) 
            }
            Statement::Var(i, exp) => {
                let val = self.eval_exp(exp);
                self.env.borrow_mut().add_ident(val, i.literal);
                Object::None
            }
            Statement::Return(exp) => {
                Object::Return(Box::new(self.eval_exp(exp)))
            }
            _=> {
                Object::None
            }
        }
    }

    fn eval_exp(&mut self, e: Expression) -> Object {
        match e {
            Expression::Function(i, block) => {
                Object::Function(i, block, self.env.clone())
            }
            Expression::FunctionCall(args, exp) => {
                let function = self.eval_exp(*exp);
            
                let mut a = Vec::new();

                for arg in args {
                    let e = self.eval_exp(arg);
                    match e {
                        Object::None => {}
                        _ => {
                            a.push(e);
                        }
                    }
                }

                self.apply_function(function, a)
            }
            Expression::If(cond, if_block, else_block) => {
                let cond = self.eval_exp(*cond);
                if self.is_true(cond) {
                    return self.eval_block(if_block)
                } else if else_block.is_some() {
                    return self.eval_block(else_block.unwrap())
                }

                Object::None
            }
            Expression::Ident(i) => {
                match self.env.borrow().read_ident(&i.literal) {
                    Some(val) => {
                        return val.clone()
                    }
                    None => {}

                }
                match self.builtin.get(&i.literal) {
                    Some(val) => {
                        return val.clone()
                    }
                    None => {}
                }

                Self::new_error(format!("{} not found in the current scope", &i.literal))
            }
            Expression::Literal(l) => {
                match l {
                    Literals::Int(i) => {
                        Object::Int(i)
                    }
                    Literals::Bool(b) => {
                        Object::Bool(b)
                    }
                    Literals::String(s) => {
                        Object::String(s)
                    }
                    Literals::Arr(arr) => {
                        let a = arr
                            .iter()
                            .map(|exp| self.eval_exp(exp.clone()))
                            .collect();

                        Object::Arr(a)
                    }
                }

            }
            Expression::Index(left, ind) => {
                let left = self.eval_exp(*left);
                let arr = match left {
                    Object::Arr(arr) => {
                        arr
                    }
                    _ => {
                        return Self::new_error("Indexed Object is not of type Array")
                    }
                };
                match self.eval_exp(*ind){
                    Object::Int(i) => {
                        arr[i as usize].clone()
                    }
                    _ => {
                        Self::new_error("Need String to Index an Array")
                    }
                }
            }
            Expression::Prefix(p, right) => {
                self.eval_prefix(p, *right)
            }
            Expression::Infix(o, left, right) => {
                self.eval_infix(o, *left, *right)
            }
            Expression::None => {
                Object::None
            }
        }
    }

    fn eval_infix(&mut self, o: Infix, left: Expression,  right: Expression) -> Object {
        let left = self.eval_exp(left);
        let right = self.eval_exp(right);
        match (left, right) {
            (Object::Int(l), Object::Int(r)) => {
                return self.calculate_int(o, l, r);
            }
            (Object::String(l), Object::String(r)) => {
                return self.calculate_string(o, l, r)
            }
            _ => {}
        }
        return Object::None
    }
    
    fn eval_prefix(&mut self, o: Prefix, right: Expression) -> Object {
        let right = self.eval_exp(right);
        return match (o, right) {
            (Prefix::Minus, Object::Int(i)) => {
                Object::Int(-1 * i)
            }
            _ => {
                Object::None
            }
        }
    }
    fn calculate_string(&mut self, operator: Infix, left: String, right: String) -> Object {
        match operator {
            Infix::Plus => {
                return Object::String(left + &right)
            }
            _ => {}
        }

        self.bool_calculation(operator, left, right)
    }

    fn apply_function(&mut self, function: Object, args: Vec<Object>) -> Object {
        match function {
            Object::Function(i, block, env) => {

                let current_env = self.env.clone();
                if i.len() != args.len() {
                    return Self::new_error(format!("Need {} Arguments got {}", i.len(), args.len()))
                }

                let extended_env = self.extend_function_env(args, i, env);

                self.env = Rc::new(RefCell::new(extended_env));

                let obj = self.eval_block(block);

                self.env = current_env;

                obj
            }
            Object::Builtin(num, func) => {
                if args.len() as i64 != num {
                    return Self::new_error(format!("Got {} Arguments but Want {}", args.len(), num))
                }

                func(args)
            }
            _ => {
                Self::new_error("Not A Valid Function Object")
            }

        }
    }
    fn extend_function_env(&mut self, args: Vec<Object>, idents: Vec<Ident>, env: Rc<RefCell<Env>>) -> Env {
        let mut new_env = Env::new_with_outer(env);

        for i in 0.. idents.len() {
            new_env.add_ident(args[i].clone(), idents[i].literal.clone());
        }

        new_env
    }


    fn is_true(&mut self, cond: Object) -> bool{
        match cond {
            Object::None | Object::Bool(false) => {
                false
            }
            _ => {
                true
            }
        }
    }

    fn calculate_int(&mut self, o: Infix, n1: i64, n2: i64) -> Object {
        match o {
            Infix::Plus  => {
                return Object::Int(n1 + n2)
            }

            Infix::Minus => {
                return Object::Int(n1 - n2)
            }

            Infix::Star => {
                return Object::Int(n1 * n2)
            }

            Infix::Slash => {
                return Object::Float(n1 as f64/ n2 as f64)
            }
            _ => {
                return self.bool_calculation(o, n1, n2)
            }

        }
    }

    fn bool_calculation<T: PartialEq + PartialOrd + fmt::Display>(&mut self, o: Infix, v1:T, v2:T) -> Object {
        match o {
            Infix::LT => {
                Object::Bool(v1 < v2)
            }
            Infix::GT => {
                Object::Bool(v1 > v2)
            }
            Infix::EQ => {
                Object::Bool(v1 == v2)
            }
            Infix::NotEQ => {
                Object::Bool(v1 != v2)
            }
            _ => {
                Self::new_error(format!("{} is not a valid Infix Operator for {} {} {}", o, v1, o, v2))
            }
        }
    }

    fn new_error<T : fmt::Display>(msg: T) -> Object {
        return Object::Error(msg.to_string())
    }
}
