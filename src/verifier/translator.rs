use z3::*;
use z3::ast::{Ast,Bool,Int};
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

    // ===============================================================
    // Integer Expressions
    // ===============================================================

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

    fn translate_int_literal(&mut self, val: usize) -> Int<'a> {
        // TODO: should fix this cast :)
        Int::from_u64(self.context,val as u64)
    }

    // ===============================================================
    // Logical Expressions
    // ===============================================================

    pub fn translate_bool(&mut self, index: usize) -> Bool<'a> {
        // Must be valid term
        assert!(index < self.heap.len());
        //
        let term = self.heap.get(index);
        match term {
            // Expressions
            Term::Binary(bop,lhs,rhs) => self.translate_bool_binary(*bop,*lhs,*rhs),
            // Literals
            Term::BoolLiteral(v) => self.translate_bool_literal(*v),
	    _ => {
		panic!("unexpected term encountered");
	    }
        }
    }

    fn translate_bool_binary(&mut self, bop: BinOp, lhs: usize, rhs: usize) -> Bool<'a> {
        // Split based on operand type
        match bop {
            BinOp::LogicalAnd|BinOp::LogicalImplies|BinOp::LogicalOr =>
                self.translate_bool_logical(bop,lhs,rhs),
            _ => self.translate_bool_relational(bop,lhs,rhs)
        }
    }

    fn translate_bool_logical(&mut self, bop: BinOp, lhs: usize, rhs: usize) -> Bool<'a> {
        // Translate lhs and rhs
        let l = self.translate_bool(lhs);
        let r = self.translate_bool(rhs);
        //
        match bop {
            BinOp::LogicalAnd => { Bool::and(self.context,&[&l,&r]) }
            BinOp::LogicalOr => { Bool::or(self.context,&[&l,&r]) }
            _ => { unreachable!() }
        }
    }

    fn translate_bool_relational(&mut self, bop: BinOp, lhs: usize, rhs: usize) -> Bool<'a> {
        // Translate lhs and rhs
        let l = self.translate_int(lhs);
        let r = self.translate_int(rhs);
        //
        match bop {
            BinOp::Equals => { l._eq(&r) }
            BinOp::NotEquals => { Ast::distinct(self.context,&[&l,&r]) }
            BinOp::LessThan => { l.lt(&r) }
            BinOp::LessThanOrEquals => { l.le(&r) }
            BinOp::GreaterThan => { l.gt(&r) }
            BinOp::GreaterThanOrEquals => { l.ge(&r) }
            //
            _ => { unreachable!() }
        }
    }

    fn translate_bool_literal(&mut self, val: bool) -> Bool<'a> {
        Bool::from_bool(self.context,val)
    }
}
