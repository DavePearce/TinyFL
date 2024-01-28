use crate::circuit::{Any,Bool,Circuit,Function,Int};
use crate::{BinOp,Environment,SyntacticHeap,Term};

use BinOp::*;

/// Responsible for translating terms in the high-level Abstract
/// Syntax Tree.
pub struct Translator<'a, C:Circuit> {
    heap: &'a SyntacticHeap,
    /// Circuit being constructed.
    context: &'a C,
    /// Maps variables from the context.
    env: &'a Environment<C>
}

impl<'a, C:Circuit> Translator<'a,C> {
    pub fn new(heap: &'a SyntacticHeap, context: &'a C, env: &'a Environment<C>) -> Self {
	Self{heap,context,env}
    }

    // =========================================================================
    // Public Interface
    // =========================================================================

    /// Translate the term at a given `index` position within the heap
    /// into an AST node.
    pub fn translate(&mut self, index: usize) -> C::Term {
        // Must be valid term
        assert!(index < self.heap.len());
        //
        let term = self.heap.get(index);
        match term {
            Term::Block(stmts) => self.translate_block(stmts),
            // // Expressions
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
    pub fn translate_bool(&mut self, index: usize) -> C::Bool {
        C::Bool::from_any(&self.translate(index))
    }

    /// Translate the term at a given `index` position within the heap
    /// into a _integer_ AST node.
    pub fn translate_int(&mut self, index: usize) -> C::Int {
        C::Int::from_any(&self.translate(index))
    }

    /// Translate the term at a given `index` position within the heap
    /// into a _sort_.  Hence, this assumes the term at `index`
    /// corresponds to a type.
    pub fn translate_type(&mut self, index: usize) -> C::Type {
        // Must be valid term
        assert!(index < self.heap.len());
        //
        let term = self.heap.get(index);
        // Types
        match term {
            Term::ArrayType(_) => todo!(),
            Term::BoolType => self.context.bool_type(),
            Term::IntType(_) => self.context.int_type(),
            Term::TupleType(_) => todo!(),
            _ => { unreachable!() }
        }
    }

    // =========================================================================
    // Private Translation Helpers
    // =========================================================================

    fn translate_block(&mut self, indices: &[usize]) -> C::Term {
        assert_eq!(indices.len(),1,"multi-statement blocks not yet supported");
        self.translate(indices[0])
    }

    // /// Translate an arbitrary binary expression.  This is done by
    // /// considering the main categories separately.
    fn translate_binary(&mut self, bop: BinOp, lhs: usize, rhs: usize) -> C::Term {
        match bop {
            // Arithmetic
            Add|Subtract|Multiply|Divide|Remainder => {
                self.translate_arithmetical(bop,lhs,rhs).to_any()
            }
            // Comparators
            LessThan|LessThanOrEquals|GreaterThan|GreaterThanOrEquals => {
                self.translate_relational(bop,lhs,rhs).to_any()
            }
            // Equality
            Equals|NotEquals => {
                self.translate_equational(bop,lhs,rhs).to_any()
            }
            // Logic
            LogicalAnd|LogicalOr|LogicalImplies => {
                self.translate_logical(bop,lhs,rhs).to_any()
            }
        }
    }

    fn translate_arithmetical(&mut self, bop: BinOp, lhs: usize, rhs: usize) -> C::Int {
        // Translate lhs and rhs
        let l = self.translate_int(lhs);
        let r = self.translate_int(rhs);

        match bop {
            // Arithmetic
            BinOp::Add => { l.add(&r) }
            BinOp::Subtract => { l.sub(&r) }
            BinOp::Multiply => { l.mul(&r) }
            BinOp::Divide => { l.div(&r) }
            BinOp::Remainder => { l.rem(&r) }
            _ => { unreachable!() }
        }
    }

    fn translate_equational(&mut self, bop: BinOp, lhs: usize, rhs: usize) -> C::Bool {
        // Translate lhs and rhs
        let l = self.translate(lhs);
        let r = self.translate(rhs);
        //
        match bop {
            BinOp::Equals => { l.eq(&r) }
            BinOp::NotEquals => { l.neq(&r) }
            //
            _ => { unreachable!() }
        }
    }

    fn translate_relational(&mut self, bop: BinOp, lhs: usize, rhs: usize) -> C::Bool {
        // Translate lhs and rhs
        let l = self.translate_int(lhs);
        let r = self.translate_int(rhs);
        //
        match bop {
            BinOp::LessThan => { l.lt(&r) }
            BinOp::LessThanOrEquals => { l.lteq(&r) }
            BinOp::GreaterThan => { l.gt(&r) }
            BinOp::GreaterThanOrEquals => { l.gteq(&r) }
            //
            _ => { unreachable!() }
        }
    }

    fn translate_logical(&mut self, bop: BinOp, lhs: usize, rhs: usize) -> C::Bool {
        // Translate lhs and rhs
        let l = self.translate_bool(lhs);
        let r = self.translate_bool(rhs);
        //
        match bop {
            BinOp::LogicalAnd => { l.and(&r) }
            BinOp::LogicalOr => { l.or(&r) }
            BinOp::LogicalImplies => { l.implies(&r) }
            _ => { unreachable!() }
        }
    }

    fn translate_ifelse(&mut self, cond: usize, lhs: usize, rhs: usize) -> C::Term {
        let c = self.translate_bool(cond);
        let l = self.translate(lhs);
        let r = self.translate(rhs);
        c.ite(&l,&r)
    }

    fn translate_static_invoke(&mut self, name: &str, args: &[usize]) -> C::Term {
        // Lookup function to invoke
        let fun = self.env.lookup_fn(name);
        // Translate arguments
        let terms : Vec<C::Term> = args.iter().map(|arg| self.translate(*arg)).collect();
        // Construct invocation
        fun.invoke(&terms)
    }

    fn translate_var(&mut self, var: &str) -> C::Term {
        self.env.lookup(var).clone()
    }

    fn translate_bool_literal(&mut self, val: bool) -> C::Term {
        let ast = self.context.from_bool(val);
        // fn
        ast.to_any()
    }

    fn translate_int_literal(&mut self, val: usize) -> C::Term {
        let ast = self.context.from_usize(val);
        // Convert to dynamic
        ast.to_any()
    }
}
