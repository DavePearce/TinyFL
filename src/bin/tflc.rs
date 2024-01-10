use std::error::Error;
use std::{fs};
use clap::{arg, Arg, ArgMatches, Command};
//
use tiny_fl::{Parser,RustPrinter,SmtLibCircuit,Verifier};

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
    let circuit : SmtLibCircuit = Verifier::new(&parser.heap).to_circuit(&terms)?;
    // Check stuff?
    Ok(true)
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
