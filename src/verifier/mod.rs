mod vcg;
mod translator;

use std::collections::{HashMap};
use z3::ast::{Dynamic};
use z3::{FuncDecl};
pub use vcg::*;


pub struct Environment<'a> {
    /// Map local variables.
    bindings: HashMap<String,Dynamic<'a>>,
    /// Bind function names to declarations.
    fn_bindings: HashMap<String, FuncDecl<'a>>,
}

impl<'a> Environment<'a> {
    pub fn new() -> Self {
        Self{bindings: HashMap::new(), fn_bindings: HashMap::new() }
    }
    pub fn alloc(&mut self, name: &str, kind: Dynamic<'a>) {
        self.bindings.insert(name.to_string(), kind);
    }
    pub fn lookup(&self, name: &str) -> &Dynamic<'a> {
        self.bindings.get(name).unwrap()
    }
    pub fn declare_fn(&mut self, decl: FuncDecl<'a>) {
        self.fn_bindings.insert(decl.name(), decl);
    }
    pub fn lookup_fn(&self, name: &str) -> &FuncDecl<'a> {
        self.fn_bindings.get(name).unwrap()
    }
}
