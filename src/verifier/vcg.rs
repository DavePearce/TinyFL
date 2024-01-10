use std::fmt;
use std::error::Error;
use crate::circuit::{Circuit,Bool,Int};
use crate::{BinOp,Function,SyntacticHeap,Term};
use super::Environment;
use super::translator::Translator;

// =============================================================================
// Verifier Error
// =============================================================================

/// Identifies a specific error arising in the verifier.  This
/// typically indicates the input program was malformed in some way
/// (for example, referred to a variable or function that was not
/// defined).  Such errors should be caught earlier in the pipeline
/// (e.g. during name resolution or type checking).
#[derive(Debug)]
pub struct VerifierError {

}

impl fmt::Display for VerifierError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for VerifierError {

}

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
pub struct Verifier<'a, C:Circuit> {
    /// Represents the original source program being verified.
    heap: &'a SyntacticHeap,
    /// The verification circuit being constructed.
    circuit: C,
    /// Name resolver
    env: Environment<C>
}

impl<'a, C:Circuit> Verifier<'a,C> {
    pub fn new(heap: &'a SyntacticHeap) -> Self {
	let env = Environment::new();
        let circuit = C::new();
        Self{heap, env, circuit}
    }

    /// Generate a circuit (i.e. a set of verification conditions) for
    /// the given set of top-level declarations in the source program.
    pub fn to_circuit(mut self, declarations: &[usize]) -> Result<C,VerifierError> {
        // Construct initial strongest postcondition.
        let precondition = self.circuit.from_bool(true);
        // Iterate all top-level declarations generating verification
        // conditions as necessary.
        for term in declarations {
            self.generate_term(*term, precondition.clone());
        }
        // Done
        Ok(self.circuit)
    }

    // ===================================================================================
    // Internal
    // ===================================================================================

    fn generate_term(&mut self, index: usize, precondition: C::Bool) -> C::Bool {
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

    // // ===================================================================================
    // // Declarations
    // // ===================================================================================

    fn generate_decl_function(&mut self, fun: &Function, mut precondition: C::Bool) -> C::Bool {
        // let precondition = self.generate_decl_precondition(fun,precondition);
        // Generate verification conditions from body
        precondition = self.generate_term(fun.body,precondition);
        // // Generate verification conditions for return types
        // self.generate_decl_checks(fun,precondition);
        // // Generate (uninterpreted) function declaration
        // let params = self.translate_param_types(&fun.params);
        // let rets = self.translate_param_types(&fun.rets);
        // let params : Vec<&Sort<'a>> = params.iter().map(|p| p).collect();
        // let fdecl = FuncDecl::new(self.context,fun.name.to_string(),&params,&rets[0]);
        // // Allocate function
        // self.env.declare_fn(fdecl);
        // //
        // Bool::from_bool(self.context,true)
        precondition
    }

    fn generate_decl_precondition(&mut self, fun: &Function, mut precondition: C::Bool) -> C::Bool {
        // // Second, extract verification conditions from body.
        // for ith in &fun.params {
        //     self.declare(ith.0,&ith.1);
        // }
        // // Update precondition to include preconditions
        // for i in fun.requires.iter() {
        //     // Translate precondition
        //     let ith = self.translate_bool(*i);
        //     // Append to list of precondition
        //     precondition = Bool::and(self.context,&[&precondition,&ith]);
        // }
        // //
        precondition
    }

    fn generate_decl_checks(&mut self, fun: &Function, mut precondition: C::Bool) {
        // let len = fun.params.len();
        // // Translate function body
        // let body = self.translate(fun.body);
        // // Allocate return parameters
        // for ith in &fun.rets {
        //     self.declare(ith.0,&ith.1);
        //     let r = self.env.lookup(&ith.1);
        //     // NOTE: the following is completely broken for functions
        //     // with multiple returns.  At this stage, I don't know how
        //     // best to resolve that.
        //     precondition = Bool::and(self.context, &[&precondition, &r._eq(&body)]);
        // }
        // // Generate postcondition checks
        // for i in fun.ensures.iter() {
        //     // Translate postcondition
        //     let ith = self.translate_bool(*i);
        //     // Emit verification condition
        //     let vcg = precondition.implies(&ith);
        //     self.vcgs.push(vcg);
        // }
    }

    // ===================================================================================
    // Statements
    // ===================================================================================

    fn generate_stmt_block(&mut self, terms: &[usize], mut precondition: C::Bool) -> C::Bool {
        for t in terms {
            precondition = self.generate_term(*t, precondition);
        }
        precondition
    }

    fn generate_stmt_assume(&mut self, expr: usize, mut precondition: C::Bool) -> C::Bool {
        // Extract verification conditions from operand
        precondition = self.generate_term(expr,precondition);
        // Translate expression
        let assumption = self.translate_bool(expr);
        // Include assumption
        precondition.and(&assumption)
    }

    fn generate_stmt_assert(&mut self, expr: usize, mut precondition: C::Bool) -> C::Bool {
        // Extract verification conditions from operand
        precondition = self.generate_term(expr,precondition);
        // Translate expression
        let assertion = self.translate_bool(expr);
        // Emit verification condition (i.e. precondition ==> assertion)
        self.circuit.assert(precondition.implies(&assertion));
        // Include assertion as assumption going forward
        precondition.and(&assertion)
    }

    // ===================================================================================
    // Expressions
    // ===================================================================================

    /// Extract verification conditions from a binary expression.
    /// Whilst some binary operators (e.g. `/`) generate verification
    /// conditions, most don't.  In all cases, we must recursively
    /// generate verification conditions for the operands.
    fn generate_expr_binary(&mut self, bop: BinOp, lhs: usize, rhs: usize, mut precondition: C::Bool) -> C::Bool {
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
    fn generate_expr_and(&mut self, lhs: usize, rhs: usize, mut precondition: C::Bool) -> C::Bool {
        // Extract vcs from left-hand side
        precondition = self.generate_term(lhs,precondition);
        // Translate left-hand side
        let l = self.translate_bool(lhs);
        // Update precondition to include the left-hand side.  The
        // reason for this is that the right-hand side is only
        // executed *when* the left-hand side is true.
        let tt_precondition = precondition.and(&l);
        // Extract vcs from right-hand side
        self.generate_term(rhs,tt_precondition);
        // FIXME: need to do some merging here!
        precondition
    }

    /// For an expression `e1 || e2` it follows (by short circuiting)
    /// that `e2` is only executed when `e1` is false.  Therefore,
    /// when executing `e2` we can safely assume that `e1` is false.
    fn generate_expr_or(&mut self, lhs: usize, rhs: usize, mut precondition: C::Bool) -> C::Bool {
        // Extract vcs from left-hand side
        precondition = self.generate_term(lhs,precondition);
        // Translate left-hand side
        let l = self.translate_bool(lhs);
        // Update precondition to include the (negated) left-hand side.
        // The reason for this is that the right-hand side is only
        // executed *when* the left-hand side is false.
        let tt_precondition = precondition.and(&l.not());
        // Extract vcs from right-hand side
        self.generate_term(rhs,tt_precondition);
        // FIXME: need to do some merging here!
        precondition
    }

    /// For an expression `e1 ==> e2` it follows (by short circuiting)
    /// that `e2` is only executed when `e1` is true.  Therefore,
    /// when executing `e2` we can safely assume that `e1` holds.
    fn generate_expr_implies(&mut self, lhs: usize, rhs: usize, mut precondition: C::Bool) -> C::Bool {
        // Extract vcs from left and right-hand sides
        precondition = self.generate_term(lhs,precondition);
        // Translate left-hand side
        let l = self.translate_bool(lhs);
        // Update precondition to include the left-hand side.  The
        // reason for this is that the right-hand side is only
        // executed *when* the left-hand side is true.
        let tt_precondition = precondition.and(&l);
        // Extract vcs from right-hand side
        self.generate_term(rhs,tt_precondition);
        //
        precondition
    }

    /// For an expression `x - y` which produces an unsigned integer,
    /// it follows that `x >= y` must hold.
    fn generate_expr_sub(&mut self, lhs: usize, rhs: usize, mut precondition: C::Bool) -> C::Bool {
        // Extract vcs from left and right-hand sides
        precondition = self.generate_term(lhs,precondition);
        precondition = self.generate_term(rhs,precondition);
        // Translate left & right-hand sides
        let l = self.translate_int(lhs);
        let r = self.translate_int(rhs);
        // Emit verification condition (i.e. lhs >= rhs)
        self.circuit.assert(precondition.implies(&l.gteq(&r)));
        // Done
        precondition
    }

    /// For an expression `x / y`, it follows that `y != 0` must hold.
    fn generate_expr_div(&mut self, lhs: usize, rhs: usize, mut precondition: C::Bool) -> C::Bool {
        // Extract vcs from left and right-hand sides
        precondition = self.generate_term(lhs,precondition);
        precondition = self.generate_term(rhs,precondition);
        // Translate left & right-hand sides
        let r = self.translate_int(rhs);
        // Emit verification condition (i.e. rhs != 0)
        self.circuit.assert(precondition.implies(&r.non_zero()));
        // Done
        precondition
    }

    /// For an expression `x % y`, it follows that `y != 0` must hold.
    fn generate_expr_rem(&mut self, lhs: usize, rhs: usize, mut precondition: C::Bool) -> C::Bool {
        // Extract vcs from left and right-hand sides
        precondition = self.generate_term(lhs,precondition);
        precondition = self.generate_term(rhs,precondition);
        // Translate left & right-hand sides
        let r = self.translate_int(rhs);
        // Emit verification condition (i.e. rhs != 0)
        self.circuit.assert(precondition.implies(&r.non_zero()));
        // Done
        precondition
    }

    /// For an expression `if e1 { e2 } else { e3 }`, it follows that
    /// `e2` is only executed when `e1` is true (and vice-versa for
    /// `e3`).  Therefore, when executing `e2` we can safely assume
    /// that `e1` holds (respectively, for `e3` that `e1` does not
    /// hold).
    fn generate_expr_ifelse(&mut self, cond: usize, lhs: usize, rhs: usize, mut precondition: C::Bool) -> C::Bool
    {
        // Extract vcs from condition
        precondition = self.generate_term(cond,precondition);
        // Translate condition
        let c = self.translate_bool(cond);
        // Update precondition to include condition.
        let mut tt_precondition = precondition.and(&c);
        let mut ff_precondition = precondition.and(&c.not());
        // Extract vcs from left-hand side
        tt_precondition = self.generate_term(lhs,tt_precondition);
        // Repeate for right-hand side
        // Extract vcs from right-hand side
        ff_precondition = self.generate_term(rhs,ff_precondition);
        // FIXME: we should try and merge both precondition.
        precondition
    }

    fn generate_expr_invoke(&mut self, _name: &str, args: &[usize], mut precondition: C::Bool) -> C::Bool {
        // Generate verification conditions from arguments
        for arg in args {
            precondition = self.generate_term(*arg,precondition);
        }
        // FIXME: generate verification condition from precondition!
        precondition
    }

    fn translate(&self, term: usize) -> C::Term {
        let mut translator = Translator::new(self.heap,&self.circuit,&self.env);
        translator.translate(term)
    }

    fn translate_bool(&self, term: usize) -> C::Bool {
        let mut translator = Translator::new(self.heap,&self.circuit,&self.env);
        translator.translate_bool(term)
    }

    fn translate_int(&self, term: usize) -> C::Int {
        let mut translator = Translator::new(self.heap,&self.circuit,&self.env);
        translator.translate_int(term)
    }

    // fn translate_param_types(&self, terms: &[(usize,String)]) -> Vec<Sort<'a>> {
    //     let mut r = Vec::new();
    //     for t in terms {
    //         r.push(self.translate_type(t.0));
    //     }
    //     r
    // }

    // fn translate_type(&self, term: usize) -> Sort<'a> {
    //     let mut translator = Translator::new(self.heap,self.context,&self.env);
    //     translator.translate_type(term)
    // }

    // fn declare(&mut self, type_index: usize, name: &str) {
    //     let term = self.heap.get(type_index);
    //     let v = match term {
    //         Term::BoolType => {
    //             let t = Bool::new_const(self.context,name);
    //             Dynamic::from_ast(&t)
    //         }
    //         Term::IntType(false) => {
    //             let t = Int::new_const(self.context,name);
    //             Dynamic::from_ast(&t)
    //         }
    //         _ => {
    //     	todo!()
    //         }
    //     };
    //     self.env.alloc(name,v);
    // }
}
