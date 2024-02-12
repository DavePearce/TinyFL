use std::error::Error;
use std::{fs};
use clap::{arg, Arg, ArgMatches, Command};
//
use tiny_fl::{Parser,RustPrinter,SyntacticHeap,Verifier};
use tiny_fl::circuit::{Circuit,Outcome,SmtLibCircuit};

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command-line arguments
    let matches = Command::new("bitl")
        .about("Bit Logic")
        .version("0.1.0")
        .subcommand_required(true)
        .arg(arg!(--verbose "Show verbose output"))
        .subcommand(
            Command::new("compile")
                .about("Compile a given source file")
                .arg(Arg::new("file").required(true))
                .visible_alias("c")
        )
        .subcommand(
            Command::new("verify")
                .about("Verify a given source file")
                .arg(Arg::new("z3-static").long("z3-static"))
                .arg(Arg::new("file").required(true))
                .visible_alias("v")
        )
        .get_matches();
    // Dispatch on outcome
    let ok = match matches.subcommand() {
        Some(("compile", args)) => compile(args),
        Some(("verify", args)) => verify(args),
        _ => unreachable!(),
    }?;
    // Determine appropriate exit code
    let exitcode = i32::from(!ok);
    // Done
    std::process::exit(exitcode);
}

fn compile(args: &ArgMatches) -> Result<bool, Box<dyn Error>> {
    // Extract the file to be compiled.
    let filename = args.get_one::<String>("file").unwrap();
    // Read file
    let contents = fs::read_to_string(filename)?;
    let mut parser = Parser::new(&contents);
    // Parse file
    let terms = match parser.parse() {
        Ok(terms) => terms,
        Err(_) => {
            panic!("failed parsing: {filename}");
        }
    };
    // Write file
    let mut rp = RustPrinter::new(&parser.heap);
    //
    for t in terms {
        rp.generate(t);
    }
    //
    println!("{}",rp.done());
    //
    Ok(true)
}

fn verify(args: &ArgMatches) -> Result<bool, Box<dyn Error>> {
    // Extract the file to be compiled.
    let filename = args.get_one::<String>("file").unwrap();
    // Check whether to use Z3 directly
    let z3_static = args.contains_id("z3-static");
    // Read file
    let contents = fs::read_to_string(filename)?;
    let mut parser = Parser::new(&contents);
    // Parse input
    let terms = match parser.parse() {
        Ok(terms) => terms,
        Err(_) => {
            panic!("failed parsing: {filename}");
        }
    };
    // Construct verifier and generate circuit
    if z3_static {
        // Z3 has been statically linked.
        z3_check(&parser.heap,&terms)
    } else {
        let smtlib = SmtLibCircuit::new();
        check(&parser.heap,&terms,smtlib)
    }
}

fn check<C:Circuit>(heap: &SyntacticHeap, terms: &[usize], circuit: C) -> Result<bool, Box<dyn Error>> {
    //
    let circuit = Verifier::new(heap,circuit).to_circuit(&terms)?;
    let mut checks = 0;
    let mut errors = 0;
    let mut warnings = 0;

    // Check conditions holds
    for outcome in circuit.check() {
        match outcome {
            Outcome::Valid => { }
            Outcome::Unknown => {
                warnings += 1;
                println!("Warning");
            }
            Outcome::Invalid => {
                // let model = solver.get_model().unwrap();
                // let r = model.eval(&x,true).unwrap();
                // println!("x = {r}");
                println!("Error");
                errors += 1;
            }
        }
        checks += 1;
    }
    println!("Verified {} check(s): {} errors / {} warnings",checks,errors,warnings);
    Ok(true)
}

// ===================================================================
// Static Z3 Feature
// ===================================================================

#[cfg(feature="z3-static")]
fn z3_check(heap: &SyntacticHeap, terms: &[usize]) -> Result<bool, Box<dyn Error>> {
    let cfg = z3::Config::new();
    let context = z3::Context::new(&cfg);
    let z3 = tiny_fl::circuit::Z3Circuit::new(&context);
    check(heap,&terms,z3)
}

#[cfg(not(feature="z3-static"))]
fn z3_check(heap: &SyntacticHeap, terms: &[usize]) -> Result<bool, Box<dyn Error>> {
    panic!("Z3 was not statically linked!")
}

// fn verify(args: &ArgMatches) -> Result<bool, Box<dyn Error>> {
//     // Extract the file to be compiled.
//     let filename = args.get_one::<String>("file").unwrap();
//     // Read file
//     let contents = fs::read_to_string(filename)?;
//     let mut parser = Parser::new(&contents);
//     // Parse input
//     let terms = match parser.parse() {
//         Ok(terms) => terms,
//         Err(_) => {
//             panic!("failed parsing: {filename}");
//         }
//     };
//     // Create Z3 Context
//     let cfg = Config::new();
//     let context = Context::new(&cfg);
//     // Construct verifier instance
//     let vcg = VcGenerator::new(&parser.heap);
//     // Extract all verification conditions
//     let vcs = vcg.generate_all(&terms);
//     // Create Z3 solver
//     let solver = Solver::new(&context);
//     let checks = vcs.len();
//     let mut errors = 0;
//     let mut warnings = 0;
//     let mut rusage = 0;
//     //
//     for mut vc in vcs {
//         //
//         println!("Checking: {vc:?}");
//         vc = vc.simplify();
//         println!("Simplified: {vc:?}");
//         // Assert it
//         solver.assert(&vc.not());
//         // Check it
//         let sr = solver.check();
//         // Check it
//         match sr {
//             SatResult::Unsat => { }
//             SatResult::Unknown => {
//                 warnings += 1;
//                 println!("Warning");
//             }
//             SatResult::Sat => {
//                 // let model = solver.get_model().unwrap();
//                 // let r = model.eval(&x,true).unwrap();
//                 // println!("x = {r}");
//                 println!("Error");
//                 errors += 1;
//             }
//         };
//         // Print resource usage
//         let cost = determine_cost(&solver,rusage);
//         println!("Resource Usage: +{:?}",cost);
//         rusage += cost;
//         // All done
//         println!("--");
//     }
//     //
//     println!("Verified {} check(s): {} errors / {} warnings",checks,errors,warnings);
//     //
//     Ok(true)
// }

// fn determine_cost(solver: &Solver, current: usize) -> usize {
//        match solver.get_statistics().value("rlimit count") {
//            Some(StatisticsValue::UInt(v)) => {
//                (v as usize) - current
//            }
//            _ => panic!("Solver doesn't support \"rlimit count\"?")
//        }
// }
