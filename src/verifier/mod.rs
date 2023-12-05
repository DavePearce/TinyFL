mod vcg;
mod translator;

use std::collections::HashMap;
use z3::ast::{Dynamic};
pub use vcg::*;
//

#[derive(Clone)]
pub struct Environment<'a> {
    /// Map local variables.
    bindings: HashMap<String,Dynamic<'a>>,
}

impl<'a> Environment<'a> {
    pub fn new() -> Self {
        Self{bindings: HashMap::new() }
    }
    pub fn alloc(&mut self, name: &str, kind: Dynamic<'a>) {
        self.bindings.insert(name.to_string(), kind);
    }
    pub fn lookup(&self, name: &str) -> &Dynamic<'a> {
        self.bindings.get(name).unwrap()
    }
}
