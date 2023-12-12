use z3::*;
use z3::ast::{Ast,Bool,Dynamic,Int};
use z3::{Context};

use crate::{BinOp,Environment,SyntacticHeap,Term};

use BinOp::*;

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

    // =========================================================================
    // Public Interface
    // =========================================================================

    /// Translate the term at a given `index` position within the heap
    /// into an AST node.
    pub fn translate(&mut self, index: usize) -> Dynamic<'a> {
        // Must be valid term
        assert!(index < self.heap.len());
        //
        let term = self.heap.get(index);
        match term {
            Term::Block(stmts) => self.translate_block(stmts),
            // Expressions
            Term::Binary(bop,lhs,rhs) => self.translate_binary(*bop,*lhs,*rhs),
            Term::Braced(lhs) => self.translate(*lhs),
            Term::IfElse{cond,tt,ff} => self.translate_ifelse(*cond,*tt,*ff),
            Term::StaticInvoke(n,args) => self.translate_static_invoke(n,args),
            Term::VarAccess(s) =>  self.translate_var(s),
            // Literals
            Term::BoolLiteral(v) => self.translate_bool_literal(*v),
            Term::IntLiteral(v) => self.translate_int_literal(*v),
	    _ => {
		panic!("unexpected term encountered {term:?}");
	    }
        }
    }

    /// Translate the term at a given `index` position within the heap
    /// into a _boolean_ AST node.
    pub fn translate_bool(&mut self, index: usize) -> Bool<'a> {
        self.translate(index).as_bool().unwrap()
    }

    /// Translate the term at a given `index` position within the heap
    /// into a _integer_ AST node.
    pub fn translate_int(&mut self, index: usize) -> Int<'a> {
        self.translate(index).as_int().unwrap()
    }

    /// Translate the term at a given `index` position within the heap
    /// into a _sort_.  Hence, this assumes the term at `index`
    /// corresponds to a type.
    pub fn translate_type(&mut self, index: usize) -> Sort<'a> {
        // Must be valid term
        assert!(index < self.heap.len());
        //
        let term = self.heap.get(index);
        // Types
        match term {
            Term::ArrayType(usize) => todo!(),
            Term::BoolType => Sort::bool(self.context),
            Term::IntType(_) => Sort::int(self.context),
            Term::TupleType(_) => todo!(),
            _ => { unreachable!() }
        }
    }

    // =========================================================================
    // Private Translation Helpers
    // =========================================================================

    fn translate_block(&mut self, indices: &[usize]) -> Dynamic<'a> {
        assert_eq!(indices.len(),1,"multi-statement blocks not yet supported");
        self.translate(indices[0])
    }

    /// Translate an arbitrary binary expression.  This is done by
    /// considering the main categories separately.
    fn translate_binary(&mut self, bop: BinOp, lhs: usize, rhs: usize) -> Dynamic<'a> {
        match bop {
            // Arithmetic
            Add|Subtract|Multiply|Divide|Remainder => self.translate_arithmetical(bop,lhs,rhs),
            // Comparators
            LessThan|LessThanOrEquals|GreaterThan|GreaterThanOrEquals => self.translate_relational(bop,lhs,rhs),
            // Equality
            Equals|NotEquals => self.translate_equational(bop,lhs,rhs),
            // Logic
            LogicalAnd|LogicalOr|LogicalImplies => self.translate_logical(bop,lhs,rhs),
            //
            _ => { unreachable!() }
        }
    }

    fn translate_arithmetical(&mut self, bop: BinOp, lhs: usize, rhs: usize) -> Dynamic<'a> {
        // Translate lhs and rhs
        let l = self.translate_int(lhs);
        let r = self.translate_int(rhs);

        let v = match bop {
            // Arithmetic
            BinOp::Add => { Int::add(self.context,&[&l,&r]) }
            BinOp::Subtract => { Int::sub(self.context,&[&l,&r]) }
            BinOp::Multiply => { Int::mul(self.context,&[&l,&r]) }
            BinOp::Divide => { l.div(&r) }
            BinOp::Remainder => { l.rem(&r) }
            _ => { unreachable!() }
        };
        // Convert to Dynamic
        Dynamic::from_ast(&v)
    }

    fn translate_equational(&mut self, bop: BinOp, lhs: usize, rhs: usize) -> Dynamic<'a> {
        // Translate lhs and rhs
        let l = self.translate(lhs);
        let r = self.translate(rhs);
        //
        let b = match bop {
            BinOp::Equals => { l._eq(&r) }
            BinOp::NotEquals => { Ast::distinct(self.context,&[&l,&r]) }
            //
            _ => { unreachable!() }
        };
        // Convert to dynamic
        Dynamic::from_ast(&b)
    }

    fn translate_relational(&mut self, bop: BinOp, lhs: usize, rhs: usize) -> Dynamic<'a> {
        // Translate lhs and rhs
        let l = self.translate_int(lhs);
        let r = self.translate_int(rhs);
        //
        let b = match bop {
            BinOp::LessThan => { l.lt(&r) }
            BinOp::LessThanOrEquals => { l.le(&r) }
            BinOp::GreaterThan => { l.gt(&r) }
            BinOp::GreaterThanOrEquals => { l.ge(&r) }
            //
            _ => { unreachable!() }
        };
        // Convert to dynamic
        Dynamic::from_ast(&b)
    }

    fn translate_logical(&mut self, bop: BinOp, lhs: usize, rhs: usize) -> Dynamic<'a> {
        // Translate lhs and rhs
        let l = self.translate_bool(lhs);
        let r = self.translate_bool(rhs);
        //
        let b = match bop {
            BinOp::LogicalAnd => { Bool::and(self.context,&[&l,&r]) }
            BinOp::LogicalOr => { Bool::or(self.context,&[&l,&r]) }
            BinOp::LogicalImplies => { l.implies(&r) }
            _ => { unreachable!() }
        };
        // Convert to dynamic
        Dynamic::from_ast(&b)
    }

    fn translate_ifelse(&mut self, cond: usize, lhs: usize, rhs: usize) -> Dynamic<'a> {
        let c = self.translate_bool(cond);
        let l = self.translate(lhs);
        let r = self.translate(rhs);
        c.ite(&l,&r)
    }

    fn translate_static_invoke(&mut self, name: &str, args: &[usize]) -> Dynamic<'a> {
        let fdecl = self.env.lookup_fn(name);
        let mut dumb = Vec::new();
        let mut vargs : Vec<&dyn Ast<'_>> = Vec::new();
        // Yes, this is a tad frustrating.
        for i in 0..args.len() {
            // FIXME: following might not be int of course :)
            dumb.push(self.translate(args[i]));
        }
        for i in 0..args.len() {
            vargs.push(&dumb[i]);
        }
        fdecl.apply(&vargs)
    }

    fn translate_var(&mut self, var: &str) -> Dynamic<'a> {
        self.env.lookup(var).clone()
    }

    fn translate_bool_literal(&mut self, val: bool) -> Dynamic<'a> {
        let ast = Bool::from_bool(self.context,val);
        // Convert
        Dynamic::from_ast(&ast)
    }

    fn translate_int_literal(&mut self, val: usize) -> Dynamic<'a> {
        // TODO: should fix this cast :)
        let ast = Int::from_u64(self.context,val as u64);
        // Convert to dynamic
        Dynamic::from_ast(&ast)
    }
}
