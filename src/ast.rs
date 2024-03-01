#[derive(Debug)]
pub enum Expr {
    Int(i64),
    Id(String),
    BinOp {
        lhs: Box<Expr>,
        op: BinOp,
        rhs: Box<Expr>,
    },
    UnOp {
        op: UnOp,
        expr: Box<Expr>,
    },
    Esc {
        stmts: Vec<Stmt>,
        expr: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum UnOp {
    Neg,
}

#[derive(Debug)]
pub enum Stmt {
    Assign { id: String, expr: Expr },
    PutInt { exprs: Vec<Expr> },
    PutCh { expr: Expr },
}

#[derive(Debug)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}
