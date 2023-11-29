use z3::*;
use z3::ast::{Bool,Int};
use z3::{Context};

use crate::{BinOp,SyntacticHeap,Term};

/// Responsible for translating terms in the high-level Abstract
/// Syntax Tree.
pub struct Translator<'a> {
    heap: &'a SyntacticHeap,
    context: &'a Context        
}

impl<'a> Translator<'a> {
    pub fn new(heap: &'a SyntacticHeap, context: &'a Context) -> Self {
	Self{heap,context}
    }
    
    pub fn translate_int(&mut self, index: usize) -> Int<'a> {
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

    pub fn translate_bool(&mut self, index: usize) -> Bool<'a> {
        todo!()
    }

    // ===============================================================
    // Expressions
    // ===============================================================

    fn translate_int_binary(&mut self, bop: BinOp, lhs: usize, rhs: usize) -> Int<'a> {
        let bytecode = match bop {
            // Arithmetic
            BinOp::Add => { todo!() }
            _ => {
                todo!()
            }
        };
    }

    fn translate_int_var(&mut self, var: &str) -> Int<'a> {
        // This is presumably where we need an environment :)
        todo!()
    }

    // ===============================================================
    // Literals
    // ===============================================================

    fn translate_int_literal(&mut self, val: usize) -> Int<'a> {
        // TODO: should fix this cast :)
        Int::from_u64(self.context,val as u64)
    }
}
