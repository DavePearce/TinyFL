/// Set of built-in operators
#[derive(Clone,Copy,Debug,PartialEq)]
pub enum NaryOp {
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

impl NaryOp {
    pub fn as_str(&self) -> &str {
        match self {
            NaryOp::Eq => "=",
            NaryOp::Neq => "!=",
            NaryOp::Add => "+",
            NaryOp::Sub => "-",
            NaryOp::Mul => "*",
            NaryOp::Div => "/",
            NaryOp::Gt => ">",
            NaryOp::GtEq => ">=",
            NaryOp::Lt => "<",
            NaryOp::LtEq => "<=",
            NaryOp::Or => "or",
            NaryOp::And => "and",
            NaryOp::Implies => "=>"
        }
    }
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum UnaryOp {
    Not
}

impl UnaryOp {
    pub fn as_str(&self) -> &str {
        match self {
            UnaryOp::Not => "not",
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
    Nary(NaryOp,Vec<Expr>),
    /// Unary Expression
    Unary(UnaryOp,Box<Expr>),
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
