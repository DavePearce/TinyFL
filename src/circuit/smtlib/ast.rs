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
    Mod,
    // Relational
    Gt,
    GtEq,
    Lt,
    LtEq,
    // Logical
    Or,
    And,
    Implies,
    Not,
    // Other
    IfThenElse
}

impl Op {
    /// Determine how many arguments are expected
    pub fn arity(&self) -> usize {
        match self {
            Op::IfThenElse => 3,
            Op::Not => 1,
            _ => usize::MAX
        }
    }
    /// Get the string representation of this operator.
    pub fn as_str(&self) -> &str {
        match self {
            Op::Eq => "=",
            Op::Neq => "!=",
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "div",
            Op::Mod => "mod",
            Op::Gt => ">",
            Op::GtEq => ">=",
            Op::Lt => "<",
            Op::LtEq => "<=",
            Op::Or => "or",
            Op::And => "and",
            Op::Implies => "=>",
            Op::Not => "not",
            Op::IfThenElse => "ite"
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
    Operator(Op,Vec<Expr>),
    /// Variable Access
    VarAccess(String)
}

#[derive(Clone,Debug,PartialEq)]
pub enum Sort {
    Bool,
    Int
}

pub enum Command {
    DeclareFun(String,Vec<Sort>,Sort),
    DeclareVar(String,Sort),
    Assert(Expr),
    CheckSat
}

pub struct Function {
    pub name: String,
    pub arity: usize
}
