mod vcg;
mod translator;

use std::collections::HashMap;
pub use vcg::*;
//

#[derive(Clone)]
struct Environment {
    // Empty for now
}

impl Environment {
    pub fn new() -> Self {
        Self{}
    }
}
