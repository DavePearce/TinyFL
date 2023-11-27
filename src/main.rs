use z3::*;
use z3::ast::{Bool,Int};
use z3::{Config, Context};

fn main() {
    // Create Z3 default context 
    let cfg = Config::new();
    let context = Context::new(&cfg);
    // Create Z3 solver
    let solver = Solver::new(&context);
    // Create a constant
    let c = Int::from_u64(&context,1);
    let x = Int::new_const(&context,"x");    
    // Assert it
    solver.assert(&x.ge(&c));
    // Check it!
    let sr = solver.check();
    // Check it
    let r = match sr {
	SatResult::Unsat => "UNSAT",
	SatResult::Unknown => "UNKNOWN",
	SatResult::Sat => {
            let model = solver.get_model().unwrap();
            let r = model.eval(&x,true).unwrap();
            println!("x = {r}");
            "SAT"
        }
    };
    println!("RESULT: {}",r);
}
