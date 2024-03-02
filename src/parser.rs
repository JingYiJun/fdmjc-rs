use lazy_static::lazy_static;
use pest::iterators::Pair;
use pest::pratt_parser::PrattParser;
use pest_derive::Parser;

use super::ast::*;

#[derive(Parser)]
#[grammar = "fdmj.pest"]
pub struct FDMJParser {}

lazy_static! {
    pub static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        PrattParser::new()
            .op(Op::infix(Add, Left) | Op::infix(Sub, Left))
            .op(Op::infix(Mul, Left) | Op::infix(Div, Left))
            .op(Op::prefix(Neg))
    };
}

impl Expr {
    pub fn parse(pair: Pair<Rule>) -> Self {
        PRATT_PARSER
            .map_primary(|pair| match pair.as_rule() {
                Rule::Int => Expr::Int(pair.as_str().parse().unwrap()),
                Rule::Id => Expr::Id(pair.as_str().to_string()),
                Rule::Expr => Self::parse(pair),
                Rule::EscExpr => {
                    let mut pairs = pair.into_inner();
                    let mut stmts = vec![];
                    for pair in pairs.next().unwrap().into_inner() {
                        stmts.push(Stmt::parse(pair));
                    }
                    let expr = Box::new(Expr::parse(pairs.next().unwrap()));
                    Expr::Esc { stmts, expr }
                }
                _ => unreachable!(),
            })
            .map_infix(|lhr, op, rhs| Expr::BinOp {
                lhs: Box::new(lhr),
                op: match op.as_rule() {
                    Rule::Add => BinOp::Add,
                    Rule::Sub => BinOp::Sub,
                    Rule::Mul => BinOp::Mul,
                    Rule::Div => BinOp::Div,
                    _ => unreachable!(),
                },
                rhs: Box::new(rhs),
            })
            .map_prefix(|op, rhs| Expr::UnOp {
                op: match op.as_rule() {
                    Rule::Neg => UnOp::Neg,
                    _ => unreachable!(),
                },
                expr: Box::new(rhs),
            })
            .parse(pair.into_inner())
    }
}

impl Stmt {
    pub fn parse(pair: Pair<Rule>) -> Self {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::AssignStmt => {
                let mut pairs = inner.into_inner();
                let id = pairs.next().unwrap().as_str().to_string();
                let expr = Expr::parse(pairs.next().unwrap());
                Stmt::Assign { id, expr }
            }
            Rule::PutIntStmt => {
                let mut exprs = vec![];
                for pair in inner.into_inner() {
                    exprs.push(Expr::parse(pair));
                }
                Stmt::PutInt { exprs }
            }
            Rule::PutChStmt => {
                let expr = Expr::parse(inner.into_inner().next().unwrap());
                Stmt::PutCh { expr }
            }
            _ => unreachable!(),
        }
    }
}

impl Program {
    pub fn parse(pair: Pair<Rule>) -> Self {
        let mut stmts = vec![];
        for pair in pair.into_inner() {
            if pair.as_rule() == Rule::EOI {
                break;
            }
            stmts.push(Stmt::parse(pair));
        }
        Program { stmts }
    }
}
