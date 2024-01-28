mod vcg;
mod translator;

use std::collections::{HashMap};
use crate::circuit::{Circuit,Function};
pub use vcg::*;


pub struct Environment<C:Circuit> {
    /// Map local variables.
    bindings: HashMap<String, C::Term>,
    /// Bind function names to declarations.
    fn_bindings: HashMap<String, C::Function>,
}

impl<C:Circuit> Environment<C> {
    pub fn new() -> Self {
        Self{bindings: HashMap::new(), fn_bindings: HashMap::new() }
    }
    pub fn alloc(&mut self, name: &str, kind: C::Term) {
        self.bindings.insert(name.to_string(), kind);
    }
    pub fn lookup(&self, name: &str) -> &C::Term {
        self.bindings.get(name).unwrap()
    }
    pub fn declare_fn(&mut self, decl: C::Function) {
        self.fn_bindings.insert(decl.name(), decl);
    }
    pub fn lookup_fn(&self, name: &str) -> &C::Function {
        self.fn_bindings.get(name).unwrap()
    }
}
