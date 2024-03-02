use crate::dsa::Table;
use core::fmt;
use std::{error::Error, fmt::Write};

use crate::ast::*;

enum ExprResult {
    Var(i64),
    Int(i64),
}

impl fmt::Display for ExprResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Var(var) => write!(f, "%{var}"),
            Self::Int(x) => write!(f, "{x}"),
        }
    }
}

impl Expr {
    fn generate(&self, s: &mut String, t: &mut Table) -> Result<ExprResult, Box<dyn Error>> {
        match self {
            Expr::Int(x) => Ok(ExprResult::Int(*x)),
            Expr::Id(id) => Ok(ExprResult::Var(t.get(id).ok_or_else(|| {
                Into::<Box<dyn Error>>::into(format!("undefined identifier {id}"))
            })?)),
            Expr::BinOp { lhs, op, rhs } => {
                let l = lhs.generate(s, t)?;
                let r = rhs.generate(s, t)?;
                let var = t.acquire_num();
                let op = match op {
                    BinOp::Add => "add",
                    BinOp::Sub => "sub",
                    BinOp::Mul => "mul",
                    BinOp::Div => "sdiv",
                };
                writeln!(s, "%{var} = {op} i64 {l}, {r}")?;
                Ok(ExprResult::Var(var))
            }
            Expr::UnOp { op, expr } => {
                let r = expr.generate(s, t)?;
                let var = t.acquire_num();
                match op {
                    UnOp::Neg => writeln!(s, "%{var} = sub i64 0, {r}").unwrap(),
                }
                Ok(ExprResult::Var(var))
            }
            Expr::Esc { stmts, expr } => {
                for stmt in stmts {
                    stmt.generate(s, t)?;
                }
                expr.generate(s, t)
            }
        }
    }
}

impl Stmt {
    fn generate(&self, s: &mut String, t: &mut Table) -> Result<(), Box<dyn Error>> {
        match self {
            Self::Assign { id, expr } => match expr.generate(s, t)? {
                ExprResult::Int(x) => {
                    let var = t.insert(id, None);
                    writeln!(s, "%{var} = add i64 0, {x}")?;
                }
                ExprResult::Var(var) => {
                    t.insert(id, Some(var));
                }
            },
            Self::PutInt { exprs } => {
                for expr in exprs {
                    let r = expr.generate(s, t)?;
                    writeln!(s, "call void @putint(i64 {r})")?;
                    writeln!(s, "call void @putch(i64 10)")?;
                }
            }
            Self::PutCh { expr } => {
                let r = expr.generate(s, t)?;
                writeln!(s, "call void @putch(i64 {r})")?;
            }
        };
        Ok(())
    }
}

impl Program {
    pub fn generate(&self, s: &mut String) -> Result<(), Box<dyn Error>> {
        let mut t = Table::new();
        for stmt in &self.stmts {
            stmt.generate(s, &mut t)?;
        }
        Ok(())
    }
}
