use super::{Circuit,Any,Bool,Int};

/// Minimal hacky circuit implementation.
pub struct SmtLibCircuit {

}

impl Circuit for SmtLibCircuit {
    type Term = usize;
    type Bool = usize;
    type Int = usize;
    type Type = usize;
    type Function = usize;

    fn new() -> Self {
        Self{}
    }

    fn from_bool(&mut self, val: bool) -> Self::Bool {
        0
    }

    fn assert(&mut self, condition: Self::Bool) {

    }
}

impl Bool for usize {
    type Any = usize;

    fn from_any(any: &Self::Any) -> Self { todo!() }
    fn not(&self) -> Self { todo!() }
    fn and(&self, other: &Self) -> Self { todo!() }
    fn or(&self, other: &Self) -> Self { todo!() }
    fn implies(&self, other: &Self) -> Self { todo!() }
}

impl Int for usize {
    type Any = usize;
    type Bool = usize;

    // Constructors
    fn from_any(any: &Self::Any) -> Self { todo!() }
    // Comparators
    fn non_zero(&self) -> Self::Bool { todo!() }
    fn lt(&self, other: &Self) -> Self::Bool { todo!() }
    fn lteq(&self, other: &Self) -> Self::Bool { todo!() }
    fn gt(&self, other: &Self) -> Self::Bool { todo!() }
    fn gteq(&self, other: &Self) -> Self::Bool { todo!() }
    // Arithmetic Operators
    fn neg(&self) -> Self { todo!() }
    fn add(&self, other: &Self) -> Self { todo!() }
    fn sub(&self, other: &Self) -> Self { todo!() }
    fn div(&self, other: &Self) -> Self { todo!() }
    fn mul(&self, other: &Self) -> Self { todo!() }
    fn rem(&self, other: &Self) -> Self { todo!() }
}

impl Any for usize {

}
