use crate::circuit;
use super::SmtLibWriter;
use super::ast::*;
use super::solver::{SmtOutcome,SmtSolver};

use super::ast::Op::*;

// =============================================================================
// SmtLib Circuit
// =============================================================================

pub struct SmtLibCircuit<'a> {
    /// Set of asserted verification conditions.
    commands: Vec<Command>,
    /// Smt Solver to use for discharging commands.
    solver: SmtSolver<'a>
}

impl<'a> SmtLibCircuit<'a> {
    pub fn new(solver: SmtSolver<'a>) -> Self {
        Self{commands: Vec::new(),solver}
    }

    pub fn discharge(&mut self, condition: Expr) {
        let r = <Expr as circuit::Bool>::not(&condition);
        self.commands.push(Command::Assert(r));
    }
}

impl<'a> circuit::Circuit for SmtLibCircuit<'a> {
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

    fn declare_bool(&mut self, name: &str) -> Self::Bool {
        self.commands.push(Command::DeclareVar(name.to_string(),Sort::Bool));
	Expr::VarAccess(name.to_string())
    }

    fn declare_int(&mut self, name: &str) -> Self::Int {
        self.commands.push(Command::DeclareVar(name.to_string(),Sort::Int));
	Expr::VarAccess(name.to_string())
    }

    fn declare_fn(&mut self, name: &str, params: &[Self::Type], rets: &[Self::Type]) -> Self::Function {
        // For now
        assert_eq!(rets.len(),1);
        self.commands.push(Command::DeclareFun(name.to_string(),params.to_vec(),rets[0].clone()));
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
        let results = self.solver.check(&self.commands);
        //
        results.iter().map(|o| {
            match o {
                SmtOutcome::Sat => circuit::Outcome::Invalid,
                SmtOutcome::Unsat => circuit::Outcome::Valid,
                _ => circuit::Outcome::Unknown
            }
        }).collect()
    }
}

// =============================================================================
// Any
// =============================================================================
impl<'a> circuit::Any for Expr {
    type Bool = Expr;

    fn eq(&self, other: &Self) -> Self::Bool {
        Expr::Operator(Eq,vec![self.clone(),other.clone()])
    }

    fn neq(&self, other: &Self) -> Self::Bool {
        Expr::Operator(Neq,vec![self.clone(),other.clone()])
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
        Expr::Operator(Not,vec![self.clone()])
    }
    fn and(&self, other: &Self) -> Self {
        Expr::Operator(And,vec![self.clone(),other.clone()])
    }
    fn or(&self, other: &Self) -> Self {
        Expr::Operator(Or,vec![self.clone(),other.clone()])
    }
    fn implies(&self, other: &Self) -> Self {
        Expr::Operator(Implies,vec![self.clone(),other.clone()])
    }
    fn ite(&self, lhs: &Self::Any, rhs: &Self::Any) -> Self::Any {
        Expr::Operator(IfThenElse,vec![self.clone(),lhs.clone(),rhs.clone()])
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
        Expr::Operator(Lt,vec![self.clone(),other.clone()])
    }
    fn lteq(&self, other: &Self) -> Self::Bool {
        Expr::Operator(LtEq,vec![self.clone(),other.clone()])
    }
    fn gt(&self, other: &Self) -> Self::Bool {
        Expr::Operator(Gt,vec![self.clone(),other.clone()])
    }
    fn gteq(&self, other: &Self) -> Self::Bool {
        Expr::Operator(GtEq,vec![self.clone(),other.clone()])
    }
    // Arithmetic OperatorOperators
    fn neg(&self) -> Self { Expr::Operator(Sub,vec![self.clone()]) }
    fn add(&self, other: &Self) -> Self {
        Expr::Operator(Add,vec![self.clone(),other.clone()])
    }
    fn sub(&self, other: &Self) -> Self {
        Expr::Operator(Sub,vec![self.clone(),other.clone()])
    }
    fn div(&self, other: &Self) -> Self {
        Expr::Operator(Div,vec![self.clone(),other.clone()])
    }
    fn mul(&self, other: &Self) -> Self {
        Expr::Operator(Mul,vec![self.clone(),other.clone()])
    }
    fn rem(&self, other: &Self) -> Self {
        Expr::Operator(Mod,vec![self.clone(),other.clone()])
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
