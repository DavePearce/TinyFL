use z3::*;
use z3::ast::{Ast,Bool,Int};
use z3::{Context};

use crate::{BinOp,Environment,SyntacticHeap,Term};

/// Responsible for translating terms in the high-level Abstract
/// Syntax Tree.
pub struct Translator<'a, 'b> {
    heap: &'a SyntacticHeap,
    context: &'a Context,
    /// Maps variables from the context
    env: &'b Environment<'a>
}

impl<'a,'b> Translator<'a,'b> {
    pub fn new(heap: &'a SyntacticHeap, context: &'a Context, env: &'b Environment<'a>) -> Self {
	Self{heap,context,env}
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
            Term::Block(stmts) => self.translate_int_block(stmts),
            // Expressions
            Term::Binary(bop,lhs,rhs) => self.translate_int_binary(*bop,*lhs,*rhs),
            Term::Braced(lhs) => self.translate_int(*lhs),
            Term::IfElse{cond,tt,ff} => self.translate_int_ifelse(*cond,*tt,*ff),
            Term::VarAccess(s) =>  self.translate_int_var(s),
            // Literals
            Term::IntLiteral(v) => self.translate_int_literal(*v),
	    _ => {
		panic!("unexpected term encountered {term:?}");
	    }
        }
    }

    fn translate_int_block(&mut self, indices: &[usize]) -> Int<'a> {
        assert_eq!(indices.len(),1,"multi-statement blocks not yet supported");
        self.translate_int(indices[0])
    }

    fn translate_int_binary(&mut self, bop: BinOp, lhs: usize, rhs: usize) -> Int<'a> {
        // Translate lhs and rhs
        let l = self.translate_int(lhs);
        let r = self.translate_int(rhs);

        match bop {
            // Arithmetic
            BinOp::Add => { Int::add(self.context,&[&l,&r]) }
            BinOp::Subtract => { Int::sub(self.context,&[&l,&r]) }
            BinOp::Multiply => { Int::mul(self.context,&[&l,&r]) }
            BinOp::Divide => { l.div(&r) }
            BinOp::Remainder => { l.rem(&r) }
            _ => { unreachable!() }
        }
    }

    fn translate_int_ifelse(&mut self, cond: usize, lhs: usize, rhs: usize) -> Int<'a> {
        let c = self.translate_bool(cond);
        let l = self.translate_int(lhs); // broken (might not be int)
        let r = self.translate_int(rhs); // broken (might not be int)
        c.ite(&l,&r)
    }

    fn translate_int_var(&mut self, var: &str) -> Int<'a> {
        self.env.lookup(var).as_int().unwrap()
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
            Term::Braced(lhs) => self.translate_bool(*lhs),
            Term::VarAccess(s) =>  self.translate_bool_var(s),
            // Literals
            Term::BoolLiteral(v) => self.translate_bool_literal(*v),
	    _ => { unreachable!(); }
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
            BinOp::LogicalImplies => { l.implies(&r) }
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

    fn translate_bool_var(&mut self, var: &str) -> Bool<'a> {
        self.env.lookup(var).as_bool().unwrap()
    }

    fn translate_bool_literal(&mut self, val: bool) -> Bool<'a> {
        Bool::from_bool(self.context,val)
    }
}
