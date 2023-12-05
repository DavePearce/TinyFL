use z3::ast::{Ast,Bool,Dynamic,Int};
use z3::{Context};
use crate::{BinOp,Function,SyntacticHeap,Term};
use super::Environment;
use super::translator::Translator;

/// Responsible for generating verification conditions necessary to
/// ensure that a given term is _well-defined_ or not.  For example,
/// consider the following statement:
///
/// ```text
///    assert xs[i] >= 0
/// ```
///
/// In this case, there is an implicit precondition that `i >=0 && i <
/// |xs|` in order for the asserted expression to be well-defined.  A
/// more complex example is the following:
///
/// ```text
///    if(i < |xs| && xs[i] >= 0)
/// ```
///
/// This is more complex because the well-definedness condition is not
/// simply `i >=0 && i < |xs|` as before.  That is, because we have
/// learned something from the left-hand side of the logical
/// conjunction.  Instead, the required condition for well-definedness
/// is `(i < |xs|) ==> (i >= 0 && i < |xs|)`.  This takes into account
/// the _short circuiting_ nature of source-level expressions.
/// Specifically, if the left-hand side of the conjunction doesn't
/// hold then it doesn't matter whether or not the right-hand side is
/// undefined.
pub struct VcGenerator<'a> {
    heap: &'a SyntacticHeap,
    /// The set of verification context.
    context: &'a Context,
    /// The set of verification conditions.
    vcgs: Vec<Bool<'a>>,
    /// Maps variables from the context
    env: Environment<'a>
}

impl<'a> VcGenerator<'a> {
    pub fn new(heap: &'a SyntacticHeap, context: &'a Context) -> Self {
	let env = Environment::new();
        let vcgs = Vec::new();
        Self{heap, env, vcgs, context}
    }

    pub fn generate_all(mut self, terms: &[usize]) -> Vec<Bool<'a>> {
        let precondition = Bool::from_bool(self.context,true);
        for term in terms {
            self.generate_term(*term, precondition.clone());
	}
        self.vcgs
    }

    // ===================================================================================
    // Internal
    // ===================================================================================

    fn generate_term(&mut self, index: usize, precondition: Bool<'a>) -> Bool<'a> {
        // Must be valid term
        assert!(index < self.heap.len());
        //
        let term = self.heap.get(index);
        match term {
            // Declarations
            Term::Function(fun) => self.generate_decl_function(fun,precondition),
            // Statements
            Term::Block(terms) => self.generate_stmt_block(terms,precondition),
            Term::Assume(e) => self.generate_stmt_assume(*e,precondition),
            Term::Assert(e) => self.generate_stmt_assert(*e,precondition),
            // Expressions
            Term::Binary(bop,lhs,rhs) => self.generate_expr_binary(*bop,*lhs,*rhs,precondition),
            Term::Braced(lhs) => self.generate_term(*lhs,precondition),
            Term::IfElse{cond,tt,ff} => self.generate_expr_ifelse(*cond,*tt,*ff,precondition),
            Term::VarAccess(_) =>  {
                // FIXME: this is wrong if the variable in question is
                // being logically asserted!
		precondition
            },
	    Term::StaticInvoke(name,args) => self.generate_expr_invoke(&name,args,precondition),
            // Literals
            Term::BoolLiteral(_) => precondition,
            Term::IntLiteral(_) => precondition,
	    _ => {
		todo!()
	    }
        }
    }

    // ===================================================================================
    // Declarations
    // ===================================================================================

    fn generate_decl_function(&mut self, fun: &Function, precondition: Bool<'a>) -> Bool<'a> {
        let precondition = self.generate_decl_precondition(fun,precondition);
        // Generate verification conditions from body
        self.generate_term(fun.body,precondition.clone());
        // Generate verification conditions for return types
	self.generate_decl_checks(fun,precondition);
	//
        Bool::from_bool(self.context,true)
    }

    fn generate_decl_precondition(&mut self, fun: &Function, mut precondition: Bool<'a>) -> Bool<'a> {
	//let mut precondition = precondition.to_vec();
        // Second, extract verification conditions from body.
        for ith in &fun.params {
            self.declare(ith.0,&ith.1);
        }
        // Update precondition to include preconditions
        for i in fun.requires.iter() {
            // Translate precondition
            let ith = self.translate_bool(*i);
            // Append to list of precondition
            precondition = Bool::and(self.context,&[&precondition,&ith]);
        }
	// //
	precondition
    }

    fn generate_decl_checks(&mut self, fun: &Function, mut precondition: Bool<'a>) {
	// let len = fun.params.len();
	// // Determine index of this function
	// let my_index = self.env.get_fn(&fun.name).unwrap();
	// // Construct return alias
	// let mut alias = vec![Bytecode::Invoke(my_index,len)];
	// for i in 0..len { alias.push(Bytecode::Var(i)); }
	// // Load returns into environment as aliases
	// for (_,n) in &fun.rets {
	//     self.env.alloc_alias(n,&alias);
	// }
	// // Generate return type checks
	// for _ in &fun.params {
	//     // BROKEN: need to consider multiple returns!
	//     self.vcs.push(Bytecode::Assert);
	//     self.vcs.push(Bytecode::Implies);
	//     self.vcs.extend_from_slice(&precondition);
	//     // BROKEN: need to consider other types
	//     self.vcs.push(Bytecode::IsUint);
	//     self.vcs.extend_from_slice(&alias);
        // }
        // // Generate postcondition checks
        // for i in fun.ensures.iter() {
        //     // Translate postcondition
        //     let ith = self.translate(*i);
        //     // Emit verification condition
	//     self.vcs.push(Bytecode::Assert);
	//     self.vcs.push(Bytecode::Implies);
	//     self.vcs.extend_from_slice(&precondition);
        //     precondition = self.and(precondition,ith.clone());
	//     self.vcs.extend(ith);
        // }
    }

    // ===================================================================================
    // Statements
    // ===================================================================================

    fn generate_stmt_block(&mut self, terms: &[usize], mut precondition: Bool<'a>) -> Bool<'a> {
        for t in terms {
            precondition = self.generate_term(*t, precondition);
        }
	precondition
    }

    fn generate_stmt_assume(&mut self, expr: usize, mut precondition: Bool<'a>) -> Bool<'a> {
	// Extract verification conditions from operand
        precondition = self.generate_term(expr,precondition);
        // Translate expression
        let assumption = self.translate_bool(expr);
	// Include assumption
	Bool::and(self.context, &[&precondition,&assumption])
    }

    fn generate_stmt_assert(&mut self, expr: usize, mut precondition: Bool<'a>) -> Bool<'a> {
	// Extract verification conditions from operand
        precondition = self.generate_term(expr,precondition);
        // Translate expression
        let assertion = self.translate_bool(expr);
        // Emit verification condition (i.e. precondition ==> assertion)
        let vcg = precondition.implies(&assertion);
        self.vcgs.push(vcg);
	// Include assertion as assumption going forward
	Bool::and(self.context, &[&precondition,&assertion])
    }

    // ===================================================================================
    // Expressions
    // ===================================================================================

    /// Extract verification conditions from a binary expression.
    /// Whilst some binary operators (e.g. `/`) generate verification
    /// conditions, most don't.  In all cases, we must recursively
    /// generate verification conditions for the operands.
    fn generate_expr_binary(&mut self, bop: BinOp, lhs: usize, rhs: usize, mut precondition: Bool<'a>) -> Bool<'a> {
        match bop {
            //
            BinOp::LogicalAnd => self.generate_expr_and(lhs,rhs,precondition),
            BinOp::LogicalOr => self.generate_expr_or(lhs,rhs,precondition),
            BinOp::LogicalImplies => self.generate_expr_implies(lhs,rhs,precondition),
            //
            BinOp::Subtract => self.generate_expr_sub(lhs,rhs,precondition),
            BinOp::Divide => self.generate_expr_div(lhs,rhs,precondition),
            BinOp::Remainder => self.generate_expr_rem(lhs,rhs,precondition),
            //
            _ => {
                precondition = self.generate_term(lhs,precondition);
                precondition = self.generate_term(rhs,precondition);
		precondition
            }
        }
    }

    /// For an expression `e1 && e2` it follows (by short circuiting)
    /// that `e2` is only executed when `e1` is true.  Therefore,
    /// when executing `e2` we can safely assume that `e1` holds.
    fn generate_expr_and(&mut self, lhs: usize, rhs: usize, mut precondition: Bool<'a>) -> Bool<'a> {
        // // Extract vcs from left-hand side
        // precondition = self.generate_term(lhs,precondition);
        // // Translate left-hand side
        // let bytecodes : Vec<Bytecode> = self.translate(lhs);
        // // Update precondition to include the left-hand side.  The
        // // reason for this is that the right-hand side is only
        // // executed *when* the left-hand side is true.
	// let tt_precondition = self.and(precondition.clone(),bytecodes);
        // // Extract vcs from right-hand side
        // self.generate_term(rhs,tt_precondition);
	// // FIXME: need to do some merging here!
	precondition
    }

    /// For an expression `e1 || e2` it follows (by short circuiting)
    /// that `e2` is only executed when `e1` is false.  Therefore,
    /// when executing `e2` we can safely assume that `e1` is false.
    fn generate_expr_or(&mut self, lhs: usize, rhs: usize, mut precondition: Bool<'a>) -> Bool<'a> {
        // // Extract vcs from left-hand side
        // precondition = self.generate_term(lhs,precondition);
        // // Translate left-hand side
        // let bytecodes : Vec<Bytecode> = self.translate(lhs);
        // // Update precondition to include the (negated) left-hand side.
        // // The reason for this is that the right-hand side is only
        // // executed *when* the left-hand side is false.
	// let mut tt_precondition = precondition.clone();
	// tt_precondition.insert(0,Bytecode::And);
        // tt_precondition.push(Bytecode::Not);
        // tt_precondition.extend(bytecodes);
        // // Extract vcs from right-hand side
        // self.generate_term(rhs,tt_precondition);
	// // FIXME: need to do some merging here!
	precondition
    }

    /// For an expression `e1 ==> e2` it follows (by short circuiting)
    /// that `e2` is only executed when `e1` is true.  Therefore,
    /// when executing `e2` we can safely assume that `e1` holds.
    fn generate_expr_implies(&mut self, lhs: usize, rhs: usize, mut precondition: Bool<'a>) -> Bool<'a> {
        // Extract vcs from left and right-hand sides
	precondition = self.generate_term(lhs,precondition);
        // Update precondition to include the left-hand side.  The
        // reason for this is that the right-hand side is only
        // executed *when* the left-hand side is true.
        let l = self.translate_bool(lhs);
        let tt_precondition = Bool::and(self.context, &[&precondition,&l]);
        //
        self.generate_term(rhs,tt_precondition);
	//
	precondition
    }

    /// For an expression `x - y` which produces an unsigned integer,
    /// it follows that `x >= y` must hold.
    fn generate_expr_sub(&mut self, lhs: usize, rhs: usize, mut precondition: Bool<'a>) -> Bool<'a> {
        // Extract vcs from left and right-hand sides
	precondition = self.generate_term(lhs,precondition);
        precondition = self.generate_term(rhs,precondition);
        // Translate left & right-hand sides
        let l = self.translate_int(lhs);
        let r = self.translate_int(rhs);
        // Emit verification condition (i.e. lhs >= rhs)
        let vcg = precondition.implies(&l.ge(&r));
        self.vcgs.push(vcg);
	// Done
	precondition
    }

    /// For an expression `x / y`, it follows that `y != 0` must hold.
    fn generate_expr_div(&mut self, lhs: usize, rhs: usize, mut precondition: Bool<'a>) -> Bool<'a> {
        // Extract vcs from left and right-hand sides
	precondition = self.generate_term(lhs,precondition);
        precondition = self.generate_term(rhs,precondition);
        // Translate left & right-hand sides
        let r = self.translate_int(rhs);
        // Emit verification condition (i.e. rhs != 0)
        let zero = Int::from_u64(self.context,0);
        let vcg = precondition.implies(&Ast::distinct(self.context,&[&r,&zero]));
        self.vcgs.push(vcg);
	// Done
	precondition
    }

    /// For an expression `x % y`, it follows that `y != 0` must hold.
    fn generate_expr_rem(&mut self, lhs: usize, rhs: usize, mut precondition: Bool<'a>) -> Bool<'a> {
        // Extract vcs from left and right-hand sides
	precondition = self.generate_term(lhs,precondition);
        precondition = self.generate_term(rhs,precondition);
        // Translate left & right-hand sides
        let r = self.translate_int(rhs);
        // Emit verification condition (i.e. rhs != 0)
        let zero = Int::from_u64(self.context,0);
        let vcg = precondition.implies(&Ast::distinct(self.context,&[&r,&zero]));
        self.vcgs.push(vcg);
	// Done
	precondition
    }

    /// For an expression `if e1 { e2 } else { e3 }`, it follows that
    /// `e2` is only executed when `e1` is true (and vice-versa for
    /// `e3`).  Therefore, when executing `e2` we can safely assume
    /// that `e1` holds (respectively, for `e3` that `e1` does not
    /// hold).
    fn generate_expr_ifelse(&mut self, cond: usize, lhs: usize, rhs: usize, mut precondition: Bool<'a>) -> Bool<'a> {
        // // Extract vcs from condition
        // precondition = self.generate_term(cond,precondition);
        // // Translate condition
        // let bytecodes : Vec<Bytecode> = self.translate(cond);
        // // Update precondition to include condition.
        // let mut tt_precondition = precondition.clone();
        // let mut ff_precondition = precondition.clone();
        // tt_precondition.insert(0,Bytecode::And);
        // tt_precondition.extend(bytecodes.clone());
        // // Extract vcs from left-hand side
        // tt_precondition = self.generate_term(lhs,tt_precondition);
        // // Repeate for right-hand side
        // ff_precondition.insert(0,Bytecode::And);
        // ff_precondition.push(Bytecode::Not);
        // ff_precondition.extend(bytecodes);
        // // Extract vcs from right-hand side
        // ff_precondition = self.generate_term(rhs,ff_precondition);
	// // FIXME: we should try and merge both precondition.
	// precondition
        todo!()
    }

    fn generate_expr_invoke(&mut self, _name: &str, args: &[usize], mut precondition: Bool<'a>) -> Bool<'a> {
	// // Generate verification conditions from arguments
	// for arg in args {
	//     precondition = self.generate_term(*arg,precondition);
	// }
	// // FIXME: generate verification condition from precondition!
	// precondition
        todo!()
    }

    fn translate_bool(&self, term: usize) -> Bool<'a> {
        let mut translator = Translator::new(self.heap,self.context,&self.env);
        translator.translate_bool(term)
    }

    fn translate_int(&self, term: usize) -> Int<'a> {
        let mut translator = Translator::new(self.heap,self.context,&self.env);
        translator.translate_int(term)
    }

    fn declare(&mut self, type_index: usize, name: &str) {
        let term = self.heap.get(type_index);
        let v = match term {
	    Term::BoolType => {
                let t = Bool::new_const(self.context,name);
                Dynamic::from_ast(&t)
	    }
	    Term::IntType(false) => {
                let t = Int::new_const(self.context,name);
                Dynamic::from_ast(&t)
	    }
	    _ => {
		todo!()
	    }
	};
        self.env.alloc(name,v);
    }
}
