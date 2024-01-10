mod smtlib;

pub use smtlib::*;

/// A circuit represents an encoding of information using logical or
/// arithmetic primitives.
pub trait Circuit {
    /// Represents a fundamental type within the circuit.
    type Type;
    /// Represents a function declaration within the circuit.
    type Function;
    type Term : Any;
    type Bool : Bool<Any=Self::Term>;
    type Int : Int<Any=Self::Term,Bool=Self::Bool>;

    /// Construct an (initially empty) circuit.
    fn new() -> Self;

    /// Construct a boolean term from a boolean value.
    fn from_bool(&mut self, val: bool) -> Self::Bool;

    /// Assert that a specific `condition` must be true for all
    /// possible interpretations of the circuit.  In effect, this
    /// places a constraint on the circuit that the given condition
    /// holds.
    fn assert(&mut self, condition: Self::Bool);
}

pub trait Any : Clone {

}

pub trait Bool : Clone {
    type Any;

    /// Construct a `Bool` from arbitrary term.  Observe that this
    /// will error if this doesn't make sense.
    fn from_any(any: &Self::Any) -> Self;

    fn not(&self) -> Self;
    fn and(&self, other: &Self) -> Self;
    fn or(&self, other: &Self) -> Self;
    fn implies(&self, other: &Self) -> Self;
}

pub trait Int : Clone {
    type Bool;
    type Any;

    /// Construct an `Int` from arbitrary term.  Observe that this
    /// will error if this doesn't make sense.
    fn from_any(any: &Self::Any) -> Self;
    //
    fn non_zero(&self) -> Self::Bool;
    fn lt(&self, other: &Self) -> Self::Bool;
    fn lteq(&self, other: &Self) -> Self::Bool;
    fn gt(&self, other: &Self) -> Self::Bool;
    fn gteq(&self, other: &Self) -> Self::Bool;
    //
    fn neg(&self) -> Self;
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn div(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn rem(&self, other: &Self) -> Self;
}

// pub trait Int {

// }
