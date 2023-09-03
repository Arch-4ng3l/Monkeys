use crate::ast::ast::{Statement, Program, Expression, Literals, Infix, Prefix, BlockStmt, Ident};
use crate::object::env::Env;
use crate::object::object::*;

pub struct Eval {
}

impl Eval {
    pub fn new() -> Self {
        return Eval{
        }
    }
    pub fn eval_program(&mut self, program: Program) {
        let mut env = Env::new(None);
        for s in program {
            self.eval_stmt(s, &mut env);
        }
    }

    fn eval_block(&mut self, block: BlockStmt, env: &mut Env) -> Object{
        for s in block { 
            let evaled = self.eval_stmt(s, env);
            match evaled {
                Object::Return(o) => {
                    println!("{}", *o);
                    return *o
                }
                _ => {
                }
            }
        }
        Object::None
    }

    fn eval_stmt(&mut self, stmt: Statement, env: &mut Env) -> Object {
        match stmt {
            Statement::ExpressionStmt(exp) => {
                self.eval_exp(exp, env) 
            }
            Statement::Var(i, exp) => {
                let val = self.eval_exp(exp, env);
                env.add_ident(val, i.literal);
                return Object::None;
            }
            Statement::Return(exp) => {
                Object::Return(Box::new(self.eval_exp(exp, env)))
            }
            _=> {
                Object::None
            }
        }
    }

    fn eval_exp(&mut self, e: Expression, env: &mut Env) -> Object {
        match e {
            Expression::Function(i, block) => {

                return Object::Function(i, block, Box::new(env))
            }
            Expression::FunctionCall(args, exp) => {
                let function = self.eval_exp(*exp, env);

                let mut a = Vec::new();
                for arg in args {
                    a.push(self.eval_exp(arg, env));
                }

                return self.apply_function(function, a)
            }
            Expression::If(cond, if_block, else_block) => {
                let cond = self.eval_exp(*cond, env);
                if self.is_true(cond) {
                    return self.eval_block(if_block, env)
                } else if else_block.is_some() {
                    return self.eval_block(else_block.unwrap(), env)
                }

                Object::None
            }
            Expression::Ident(i) => {
                let val = env.read_ident(i.literal);
                if val.is_some() {
                    return val.unwrap().clone();
                }
                Object::None
            }
            Expression::Literal(l) => {
                match l {
                    Literals::Int(i) => {
                        Object::Int(i)
                    }
                    Literals::Bool(b) => {
                        Object::Bool(b)
                    }
                    _ => {
                        Object::None
                    }
                }

            }
            Expression::Prefix(p, right) => {
                self.eval_prefix(p, *right, env)
            }
            Expression::Infix(o, left, right) => {
                self.eval_infix(o, *left, *right, env)
            }
            _ => {
                Object::None
            }
        }
    }

    fn eval_infix(&mut self, o: Infix, left: Expression,  right: Expression, env: &mut Env) -> Object {
        let left = self.eval_exp(left, env);
        let right = self.eval_exp(right, env);
        match (left, right) {
            (Object::Int(l), Object::Int(r)) => {
                return self.calculate_int(o, l, r);
            }
            _ => {}
        }
        return Object::None
    }
    
    fn eval_prefix(&mut self, o: Prefix, right: Expression, env: &mut Env) -> Object {
        let right = self.eval_exp(right, env);
        return match (o, right) {
            (Prefix::Minus, Object::Int(i)) => {
                Object::Int(-1 * i)
            }
            _ => {
                Object::None
            }
        }
    }

    fn apply_function(&mut self, function: Object, args: Vec<Object>) -> Object {
        match function {
            Object::Function(i, block, mut env) => {
                let mut extended_env = self.extend_function_env(args, i, &mut env);

                self.eval_block(block, &mut extended_env)
            }
            _ => {
                Object::None
            }

        }
    }
    fn extend_function_env(&mut self, args: Vec<Object>, idents: Vec<Ident>, env: &mut Env) -> Env {
        let mut new_env = Env::new(Some(*env));

        for i in 0.. idents.len() {
            new_env.add_ident(args[i].clone(), idents[i].literal.clone());
        }

        new_env
    }

    fn eval_exp_list(&mut self, exps: Vec<Expression>, env: &mut Env) -> Vec<Object> {
        let mut objs = Vec::new();

        for exp in exps {
            objs.push(self.eval_exp(exp, env))
        }

        objs

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

    fn bool_calculation<T: PartialEq + PartialOrd>(&mut self, o: Infix, v1:T, v2:T) -> Object {
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
                Object::None
            }
        }
    }
}
