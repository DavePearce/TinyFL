use z3::*;
use z3::ast::{Bool,Int};
use z3::{Context};

use crate::{BinOp,SyntacticHeap,Term};

/// Responsible for translating terms in the high-level Abstract
/// Syntax Tree into sequences of low-level bytecodes suitable for the
/// automated theorem prover.  Such programs are strictly functional,
/// using only recursion for loops.
pub struct Translator<'a> {
    heap: &'a SyntacticHeap,
    context: &'a Context        
}

impl<'a> Translator<'a> {
    pub fn new(heap: &'a SyntacticHeap, context: &'a Context) -> Self {
	Self{heap,context}
    }
    
    pub fn translate_int(&mut self, index: usize) {
        // Must be valid term
        assert!(index < self.heap.len());
        //
        let term = self.heap.get(index);
        match term {
            // Expressions
            Term::Binary(bop,lhs,rhs) => self.translate_int_binary(*bop,*lhs,*rhs),
            Term::Braced(lhs) => self.translate_int(*lhs),
            Term::VarAccess(s) =>  self.translate_int_var(s),
            // Literals
            Term::IntLiteral(v) => self.translate_int_literal(*v),
	    _ => {
		panic!("unexpected term encountered");
	    }
        }
    }

    // ===============================================================
    // Expressions
    // ===============================================================

    fn translate_int_binary(&mut self, bop: BinOp, lhs: usize, rhs: usize) {
        let bytecode = match bop {
            // Arithmetic
            BinOp::Add => { todo!() }
            _ => {
                todo!()
            }
        };
    }

    fn translate_int_var(&mut self, var: &str) {
        todo!()
    }

    // ===============================================================
    // Literals
    // ===============================================================

    fn translate_int_literal(&mut self, val: usize) {
        todo!()
    }
}
