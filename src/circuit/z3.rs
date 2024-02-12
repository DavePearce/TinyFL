use z3::ast::{Ast,Dynamic};
use z3::*;
use super::{Circuit,Any,Bool,Function,Int,Outcome,Type};

type Z3Any<'a> = z3::ast::Dynamic<'a>;
type Z3Bool<'a> = z3::ast::Bool<'a>;
type Z3Int<'a> = z3::ast::Int<'a>;
type Z3Type<'a> = z3::Sort<'a>;
type Z3Func<'a> = z3::FuncDecl<'a>;

pub struct Z3Circuit<'a> {
    context: &'a Context,
    /// Set of asserted verification conditions.
    conditions: Vec<Z3Bool<'a>>
}

impl<'a> Z3Circuit<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self{context, conditions: Vec::new()}
    }

    pub fn len(&self) -> usize {
        self.conditions.len()
    }

    pub fn check(&self, ith: usize) -> Outcome {
        let vc = &self.conditions[ith];
        let solver = Solver::new(&self.context);
        // Assert it
        solver.assert(&vc.not());
        // Check it
        match solver.check() {
             SatResult::Unsat => Outcome::Valid,
             SatResult::Sat => Outcome::Invalid,
             SatResult::Unknown => Outcome::Unknown
        }
    }

    pub fn discharge(&mut self, condition: Z3Bool<'a>) {
        self.conditions.push(condition);
    }
}

/// Minimal hacky circuit implementation.
impl<'a> Circuit for Z3Circuit<'a> {
    type Term = Z3Any<'a>;
    type Bool = Z3Bool<'a>;
    type Int = Z3Int<'a>;
    type Type = Z3Type<'a>;
    type Function = Z3Func<'a>;

    fn from_bool(&self, val: bool) -> Self::Bool {
        Z3Bool::from_bool(&self.context,val)
    }

    fn from_usize(&self, val: usize) -> Self::Int {
        Z3Int::from_u64(&self.context,val as u64)
    }

    fn declare_bool(&self, name: &str) -> Self::Bool {
        Z3Bool::new_const(self.context,name)
    }

    fn declare_int(&self, name: &str) -> Self::Int {
        Z3Int::new_const(self.context,name)
    }

    fn declare_fn(&self, name: &str, params: &[Self::Type], rets: &[Self::Type]) -> Self::Function {
        // Sanity check for now
        assert!(rets.len() <= 1);
        let params : Vec<&Sort<'a>> = params.iter().map(|p| p).collect();
        //
        Z3Func::new(self.context,name.to_string(),&params,&rets[0])
    }

    fn bool_type(&self) -> Self::Type {
        Sort::bool(self.context)
    }

    fn int_type(&self) -> Self::Type {
        Sort::int(self.context)
    }

    fn assert(&mut self, condition: Self::Bool) {
        Z3Circuit::discharge(self,condition);
    }

    fn check(&self) -> Vec<Outcome> {
        let mut outcomes = Vec::new();
        for i in 0..self.conditions.len() {
            outcomes.push(self.check(i));
        }
        outcomes
    }
}

// =============================================================================
// Z3 Any
// =============================================================================
impl<'a> Any for Z3Any<'a> {
    type Bool = Z3Bool<'a>;

    fn eq(&self, other: &Self) -> Self::Bool {
        self._eq(other)
    }

    fn neq(&self, other: &Self) -> Self::Bool {
        Dynamic::distinct(self.get_ctx(),&[self,other])
    }
}

// =============================================================================
// Z3 Boolean
// =============================================================================

impl<'a> Bool for Z3Bool<'a> {
    type Any = Z3Any<'a>;

    fn from_any(any: &Self::Any) -> Self {
        any.as_bool().unwrap()
    }
    fn to_any(&self) -> Self::Any {
        Z3Any::from_ast(self)
    }
    fn not(&self) -> Self {
        self.not()
    }
    fn and(&self, other: &Self) -> Self {
        Self::and(self.get_ctx(),&[self,other])
    }
    fn or(&self, other: &Self) -> Self {
        Self::or(self.get_ctx(),&[self,other])
    }
    fn implies(&self, other: &Self) -> Self {
        self.implies(other)
    }
    fn ite(&self, lhs: &Self::Any, rhs: &Self::Any) -> Self::Any {
        self.ite(lhs,rhs)
    }
}

// =============================================================================
// Z3 Int
// =============================================================================
impl<'a> Int for Z3Int<'a> {
    type Any = Z3Any<'a>;
    type Bool = Z3Bool<'a>;

    // Constructors
    fn from_any(any: &Self::Any) -> Self {
        any.as_int().unwrap()
    }
    //
    fn to_any(&self) -> Self::Any {
        Z3Any::from_ast(self)
    }
    // Comparators
    fn non_zero(&self) -> Self::Bool { todo!() }
    fn lt(&self, other: &Self) -> Self::Bool { self.lt(other) }
    fn lteq(&self, other: &Self) -> Self::Bool { self.le(other) }
    fn gt(&self, other: &Self) -> Self::Bool { self.gt(other) }
    fn gteq(&self, other: &Self) -> Self::Bool { self.ge(other) }
    // Arithmetic Operators
    fn neg(&self) -> Self { std::ops::Neg::neg(self) }
    fn add(&self, other: &Self) -> Self { std::ops::Add::add(self,other) }
    fn sub(&self, other: &Self) -> Self { std::ops::Sub::sub(self,other) }
    fn div(&self, other: &Self) -> Self { std::ops::Div::div(self,other) }
    fn mul(&self, other: &Self) -> Self { std::ops::Mul::mul(self,other) }
    fn rem(&self, other: &Self) -> Self { std::ops::Rem::rem(self,other) }
}

// =============================================================================
// Z3 Type
// =============================================================================
impl<'a> Type for Z3Type<'a> {

}

// =============================================================================
// Z3 Function
// =============================================================================

impl<'a> Function for Z3Func<'a> {
    type Any = Z3Any<'a>;

    fn name(&self) -> String {
        self.name()
    }

    fn invoke(&self, args: &[Self::Any]) -> Self::Any {
        let mut vargs : Vec<&dyn Ast<'_>> = Vec::new();
        // NOTE: I'm not sure why the Z3 API requires the arguments in
        // this form.
        for i in 0..args.len() { vargs.push(&args[i]); }
        //
        self.apply(&vargs)
    }
}
