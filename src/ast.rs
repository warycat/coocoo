use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};
use walrus::ir::*;
use walrus::FunctionId;
use walrus::InstrSeqBuilder;

pub trait Compile {
    fn compile(
        &self,
        builder: &mut InstrSeqBuilder,
        local_ids: &HashMap<String, LocalId>,
        function_ids: &HashMap<String, FunctionId>,
    );
}

pub enum Expr {
    Number(f64),
    Variable(String),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Call(String, Vec<Box<Expr>>),
    Error,
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Self::Number(n) => write!(fmt, "{:?}", n),
            Self::Variable(ref name) => write!(fmt, "{:?}", name),
            Self::Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Self::Call(ref name, ref exprs) => write!(fmt, "{:?} {:?}", name, exprs),
            Self::Error => write!(fmt, "error"),
        }
    }
}

impl Compile for Expr {
    fn compile(
        &self,
        builder: &mut InstrSeqBuilder,
        local_ids: &HashMap<String, LocalId>,
        function_ids: &HashMap<String, FunctionId>,
    ) {
        use self::Expr::*;
        match *self {
            Number(n) => {
                builder.f64_const(n);
            }
            Op(ref l, op, ref r) => {
                l.compile(builder, local_ids, function_ids);
                r.compile(builder, local_ids, function_ids);
                op.compile(builder, local_ids, function_ids);
            }
            Variable(ref name) => {
                let id = local_ids[name];
                builder.local_get(id);
            }
            Call(ref name, ref exprs) => {
                for expr in exprs {
                    expr.compile(builder, local_ids, function_ids);
                    // builder.local_set();
                }
                let id = function_ids[name];
                builder.call(id);
            }
            _ => {}
        }
    }
}

#[derive(Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

impl Debug for Opcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Opcode::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}

impl Compile for Opcode {
    fn compile(
        &self,
        builder: &mut InstrSeqBuilder,
        local_ids: &HashMap<String, LocalId>,
        function_ids: &HashMap<String, FunctionId>,
    ) {
        use self::Opcode::*;
        match *self {
            Mul => builder.binop(BinaryOp::F64Mul),
            Div => builder.binop(BinaryOp::F64Div),
            Add => builder.binop(BinaryOp::F64Add),
            Sub => builder.binop(BinaryOp::F64Sub),
        };
    }
}

#[derive(Debug)]
pub struct Prototype {
    pub name: String,
    pub params: Vec<String>,
}

impl Prototype {
    pub fn new(name: String, params: Vec<String>) -> Self {
        Prototype { name, params }
    }
}

impl Compile for Prototype {
    fn compile(
        &self,
        builder: &mut InstrSeqBuilder,
        local_ids: &HashMap<String, LocalId>,
        function_ids: &HashMap<String, FunctionId>,
    ) {
        for param in &self.params {
            let id = local_ids[param];
            // builder.local_set(id);
        }
    }
}

#[derive(Debug)]
pub struct Function {
    pub prototype: Prototype,
    pub expr: Box<Expr>,
}

impl Function {
    pub fn new(prototype: Prototype, expr: Box<Expr>) -> Self {
        Function { prototype, expr }
    }
}

impl Compile for Function {
    fn compile(
        &self,
        builder: &mut InstrSeqBuilder,
        local_ids: &HashMap<String, LocalId>,
        function_ids: &HashMap<String, FunctionId>,
    ) {
        self.prototype.compile(builder, local_ids, function_ids);
        self.expr.as_ref().compile(builder, local_ids, function_ids);
    }
}
