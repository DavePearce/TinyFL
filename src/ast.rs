// ===================================================================
// Binary Operators
// ===================================================================

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum BinOp {
    // Arithmetic
    Add,
    Subtract,
    Divide,
    Multiply,
    Remainder,
    // Comparators
    Equals,
    NotEquals,
    LessThan,
    LessThanOrEquals,
    GreaterThan,
    GreaterThanOrEquals,
    // Logical
    LogicalAnd,
    LogicalImplies,
    LogicalOr
}

// ===================================================================
// Term
// ===================================================================

#[derive(Clone,Debug)]
pub enum Term {
    // Declarations
    Function(Function),
    // Statements
    Assert(usize),
    Assume(usize),
    Block(Vec<usize>),
    // Expressions
    ArrayAccess{src: usize, index: usize},
    ArrayGenerator(usize,usize),
    ArraySlice{src: usize, start: usize, end: usize},
    ArrayLength(usize),
    ArrayConstructor(Vec<usize>),
    Binary(BinOp,usize,usize),
    BoolLiteral(bool),
    Braced(usize),
    IntLiteral(usize),
    IfElse{cond: usize, tt: usize, ff: usize},
    VarAccess(String),
    StaticInvoke(String,Vec<usize>),
    TupleAccess(usize,usize),
    TupleConstructor(Vec<usize>),
    // Types
    ArrayType(usize),
    BoolType,
    IntType(bool),
    TupleType(Vec<usize>)
}

// ===================================================================
// Function
// ===================================================================

#[derive(Debug,Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<(usize,String)>,
    pub rets: Vec<(usize,String)>,    
    pub requires: Vec<usize>,
    pub ensures: Vec<usize>,
    pub body: usize
}

// ===================================================================
// SyntacticHeap
// ===================================================================

/// Simplest possible implementation of a syntactic heap.
pub struct SyntacticHeap{
    nodes: Vec<Term>
}

impl SyntacticHeap {
    pub fn new() -> Self {
        SyntacticHeap{nodes: Vec::new()}
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn get(&self, index: usize) -> &Term {
        &self.nodes[index]
    }

    /// Allocate a new term into this heap
    pub fn alloc(&mut self, term: Term) -> usize {
        let index = self.len();
        self.nodes.push(term);
        index
    }

    pub fn to_ref<'a>(&'a self, index: usize) -> SyntacticRef<'a> {
        SyntacticRef{heap:self,index}
    }
}

// ===================================================================
// SyntacticRef
// ===================================================================

pub struct SyntacticRef<'a> {
    pub heap: &'a SyntacticHeap,
    pub index: usize
}
