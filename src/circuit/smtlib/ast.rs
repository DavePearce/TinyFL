/// Set of built-in operators
#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Op {
    Eq,
    Neq,
    // Arithmetical
    Add,
    Sub,
    Mul,
    Div,
    // Relational
    Gt,
    GtEq,
    Lt,
    LtEq,
    // Logical
    Or,
    And,
    Implies
}

impl Op {
    pub fn as_str(&self) -> &str {
        match self {
            Op::Eq => "=",
            Op::Neq => "!=",
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
            Op::Gt => ">",
            Op::GtEq => ">=",
            Op::Lt => ">",
            Op::LtEq => "<=",
            Op::Or => "or",
            Op::And => "and",
            Op::Implies => "=>"
        }
    }
}

#[derive(Clone,Debug,PartialEq)]
pub enum Expr {
    /// Int Literal
    Integer(usize),
    /// Boolean Literal
    Boolean(bool),
    /// Nary Expression
    Nary(Op,Vec<Expr>),
    /// Variable Access
    VarAccess(String)
}

#[derive(Clone,Debug,PartialEq)]
pub enum Sort {
    Bool,
    Int
}

pub enum Command {
    Assert(Expr),
    CheckSat
}

pub struct Function {
    pub name: String,
    pub arity: usize
}
