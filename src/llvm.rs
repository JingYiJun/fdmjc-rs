use crate::dsa::Table;

use crate::ast::*;

enum ExprResult {
    Var(i64),
    Int(i64),
}

impl ExprResult {
    pub fn to_string(self: &Self) -> String {
        match self {
            Self::Var(var) => format!("%{var}"),
            Self::Int(x) => format!("{x}"),
        }
    }
}

impl Expr {
    fn generate(self: &Self, s: &mut String, t: &mut Table) -> ExprResult {
        match self {
            Expr::Int(x) => ExprResult::Int(*x),
            Expr::Id(id) => {
                ExprResult::Var(t.get(id).expect(&format!("undefined identifier {id}")))
            }
            Expr::BinOp { lhs, op, rhs } => {
                let l = lhs.generate(s, t).to_string();
                let r = rhs.generate(s, t).to_string();
                let var = t.acquire_num();
                let op = match op {
                    BinOp::Add => "add",
                    BinOp::Sub => "sub",
                    BinOp::Mul => "mul",
                    BinOp::Div => "sdiv",
                };
                s.push_str(&format!("%{var} = {op} i64 {l}, {r}\n"));
                ExprResult::Var(var)
            }
            Expr::UnOp { op, expr } => {
                let r = expr.generate(s, t).to_string();
                let var = t.acquire_num();
                match op {
                    UnOp::Neg => s.push_str(&format!("%{var} = sub i64 0, {r}\n")),
                }
                ExprResult::Var(var)
            }
            Expr::Esc { stmts, expr } => {
                for stmt in stmts {
                    stmt.generate(s, t);
                }
                expr.generate(s, t)
            }
        }
    }
}

impl Stmt {
    fn generate(self: &Self, s: &mut String, t: &mut Table) {
        match self {
            Self::Assign { id, expr } => match expr.generate(s, t) {
                ExprResult::Int(x) => {
                    let var = t.insert(id, None);
                    s.push_str(&format!("%{var} = add i64 0, {x}\n"));
                }
                ExprResult::Var(var) => {
                    t.insert(id, Some(var));
                }
            },
            Self::PutInt { exprs } => {
                for expr in exprs {
                    let r = expr.generate(s, t).to_string();
                    s.push_str(&format!(
                        "call void @putint(i64 {r})\ncall void @putch(i64 10)\n"
                    ));
                }
            }
            Self::PutCh { expr } => {
                let r = expr.generate(s, t).to_string();
                s.push_str(&format!("call void @putch(i64 {r})\n"))
            }
        }
    }
}

impl Program {
    pub fn generate(self: &Self, s: &mut String) {
        let mut t = Table::new();
        for stmt in &self.stmts {
            stmt.generate(s, &mut t);
        }
    }
}
