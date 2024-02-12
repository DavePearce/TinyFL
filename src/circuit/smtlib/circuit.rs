use std::io;
use crate::circuit;
use super::SmtLibWriter;
use super::ast::*;

// =============================================================================
// SmtLib Circuit
// =============================================================================

pub struct SmtLibCircuit {
    /// Set of asserted verification conditions.
    commands: Vec<Command>
}

impl SmtLibCircuit {
    pub fn new() -> Self {
        Self{commands: Vec::new()}
    }

    pub fn discharge(&mut self, condition: Expr) {
        self.commands.push(Command::Assert(condition));
    }
}

impl circuit::Circuit for SmtLibCircuit {
    type Term = Expr;
    type Bool = Expr;
    type Int = Expr;
    type Type = Sort;
    type Function = Function;

    fn from_bool(&self, val: bool) -> Self::Bool {
        Expr::Boolean(val)
    }

    fn from_usize(&self, val: usize) -> Self::Int {
        Expr::Integer(val)
    }

    fn declare_bool(&self, name: &str) -> Self::Bool {
	Expr::VarAccess(name.to_string())
    }

    fn declare_int(&self, name: &str) -> Self::Int {
	Expr::VarAccess(name.to_string())
    }

    fn declare_fn(&self, name: &str, params: &[Self::Type], rets: &[Self::Type]) -> Self::Function {
        Function{name: name.to_string(),arity:params.len()}
    }

    fn bool_type(&self) -> Self::Type {
        Sort::Bool
    }

    fn int_type(&self) -> Self::Type {
        Sort::Int
    }

    fn assert(&mut self, condition: Self::Bool) {
        SmtLibCircuit::discharge(self,condition);
    }

    fn check(&self) -> Vec<circuit::Outcome> {
        let mut writer = SmtLibWriter::new(io::stdout());
        writer.write(&self.commands);
        vec![]
    }
}

// =============================================================================
// Any
// =============================================================================
impl<'a> circuit::Any for Expr {
    type Bool = Expr;

    fn eq(&self, other: &Self) -> Self::Bool {
        Expr::Nary(Op::Eq,vec![self.clone(),other.clone()])
    }

    fn neq(&self, other: &Self) -> Self::Bool {
        Expr::Nary(Op::Neq,vec![self.clone(),other.clone()])
    }
}

// =============================================================================
// Boolean
// =============================================================================
impl<'a> circuit::Bool for Expr {
    type Any = Expr;

    fn from_any(any: &Self::Any) -> Self {
        any.clone()
    }
    fn to_any(&self) -> Self::Any {
        self.clone()
    }
    fn not(&self) -> Self {
        todo!()
    }
    fn and(&self, other: &Self) -> Self {
        Expr::Nary(Op::And,vec![self.clone(),other.clone()])
    }
    fn or(&self, other: &Self) -> Self {
        Expr::Nary(Op::Or,vec![self.clone(),other.clone()])
    }
    fn implies(&self, other: &Self) -> Self {
        Expr::Nary(Op::Implies,vec![self.clone(),other.clone()])
    }
    fn ite(&self, lhs: &Self::Any, rhs: &Self::Any) -> Self::Any {
        todo!()
    }
}

// =============================================================================
// Int
// =============================================================================
impl circuit::Int for Expr {
    type Any = Expr;
    type Bool = Expr;

    // Constructors
    fn from_any(any: &Self::Any) -> Self {
        any.clone()
    }
    //
    fn to_any(&self) -> Self::Any {
        self.clone()
    }
    // Comparators
    fn non_zero(&self) -> Self::Bool { todo!() }
    fn lt(&self, other: &Self) -> Self::Bool {
        Expr::Nary(Op::Lt,vec![self.clone(),other.clone()])
    }
    fn lteq(&self, other: &Self) -> Self::Bool {
        Expr::Nary(Op::LtEq,vec![self.clone(),other.clone()])
    }
    fn gt(&self, other: &Self) -> Self::Bool {
        Expr::Nary(Op::Gt,vec![self.clone(),other.clone()])
    }
    fn gteq(&self, other: &Self) -> Self::Bool {
        Expr::Nary(Op::GtEq,vec![self.clone(),other.clone()])
    }
    // Arithmetic Operators
    fn neg(&self) -> Self { todo!() }
    fn add(&self, other: &Self) -> Self {
        Expr::Nary(Op::Add,vec![self.clone(),other.clone()])
    }
    fn sub(&self, other: &Self) -> Self {
        Expr::Nary(Op::Sub,vec![self.clone(),other.clone()])
    }
    fn div(&self, other: &Self) -> Self {
        Expr::Nary(Op::Div,vec![self.clone(),other.clone()])
    }
    fn mul(&self, other: &Self) -> Self {
        Expr::Nary(Op::Mul,vec![self.clone(),other.clone()])
    }
    fn rem(&self, other: &Self) -> Self {
        todo!()
    }
}

// =============================================================================
// Type
// =============================================================================
impl circuit::Type for Sort {

}

// =============================================================================
// Function
// =============================================================================
impl circuit::Function for Function {
    type Any = Expr;

    fn name(&self) -> String {
        self.name.to_string()
    }

    fn invoke(&self, args: &[Self::Any]) -> Self::Any {
        todo!()
    }
}
