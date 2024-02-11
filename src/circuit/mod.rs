mod z3;

pub use z3::*;

pub enum Outcome {
    /// Indicates a given condition holds for all interpretations.
    Valid,
    /// Indicates a given condition does not hold for all
    /// interpretations (ideally it would include a counter-example).
    Invalid,
    /// Unknown outcome (e.g. becaues of a timeout).
    Unknown
}

/// A circuit represents an encoding of information using logical or
/// arithmetic primitives.
pub trait Circuit {
    /// Represents a fundamental type within the circuit.
    type Type : Type;
    /// Represents a function declaration within the circuit.
    type Term : Any<Bool=Self::Bool>;
    type Bool : Bool<Any=Self::Term>;
    type Int : Int<Any=Self::Term,Bool=Self::Bool>;
    type Function : Function<Any=Self::Term>;

    /// Declare a boolean variable
    fn declare_bool(&self, name: &str) -> Self::Bool;

    /// Declare an integer variable
    fn declare_int(&self, name: &str) -> Self::Int;

    /// Declare an (uninterpreted) function.
    fn declare_fn(&self, name: &str, params: &[Self::Type], returns: &[Self::Type]) -> Self::Function;

    /// Construct a boolean term from a boolean value.
    fn from_bool(&self, val: bool) -> Self::Bool;

    /// Construct a boolean term from a boolean value.
    fn from_usize(&self, val: usize) -> Self::Int;

    /// Construct a boolean type
    fn bool_type(&self) -> Self::Type;

    /// Construct an integer type
    fn int_type(&self) -> Self::Type;

    /// Assert that a specific `condition` must be true for all
    /// possible interpretations of the circuit.  In effect, this
    /// places a constraint on the circuit that the given condition
    /// holds.
    fn assert(&mut self, condition: Self::Bool);
}

pub trait Any : Clone {
    type Bool;

    fn eq(&self, other: &Self) -> Self::Bool;
    fn neq(&self, other: &Self) -> Self::Bool;
}

pub trait Bool : Clone {
    type Any;

    /// Construct a `Bool` from arbitrary term.  Observe that this
    /// will error if this doesn't make sense.
    fn from_any(any: &Self::Any) -> Self;
    /// Convert a `bool` into an arbitrary term.
    fn to_any(&self) -> Self::Any;

    fn not(&self) -> Self;
    fn and(&self, other: &Self) -> Self;
    fn or(&self, other: &Self) -> Self;
    fn implies(&self, other: &Self) -> Self;
    fn ite(&self, lhs: &Self::Any, rhs: &Self::Any) -> Self::Any;
}

pub trait Int : Clone {
    type Any;
    type Bool;

    /// Construct an `Int` from arbitrary term.  Observe that this
    /// will error if this doesn't make sense.
    fn from_any(any: &Self::Any) -> Self;
    /// Convert an `int` into an arbitrary term.
    fn to_any(&self) -> Self::Any;

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

pub trait Type : Clone {
    // What goes here?
}

pub trait Function {
    type Any;

    /// Get the name of this function.
    fn name(&self) -> String;

    /// Construct a term representing an invocation of this function
    /// with the given number of arguments.
    fn invoke(&self,args: &[Self::Any]) -> Self::Any;
}
