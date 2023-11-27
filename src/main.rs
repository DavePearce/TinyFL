use z3::*;
use z3::ast::Bool;
use z3::{Config, Context};

fn main() {
    // Create Z3 default context 
    let cfg = Config::new();
    let context = Context::new(&cfg);
    // Create Z3 solver
    let solver = Solver::new(&context);
    // Create a constant
    let b = Bool::from_bool(&context,false);
    // Assert it
    solver.assert(&b);
    // Check it!
    let sr = solver.check();
    // Check it
    let r = match sr {
	SatResult::Unsat => "UNSAT",
	SatResult::Sat => "SAT",
	SatResult::Unknown => "UNKNOWN",	
    };
    println!("RESULT: {}",r);
}
