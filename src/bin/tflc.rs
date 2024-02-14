use std::error::Error;
use std::{fs};
use std::ffi::OsString;
use std::path::PathBuf;
use clap::{arg, Arg, ArgMatches, Command, value_parser};
//
use tiny_fl::{Parser,RustPrinter,SyntacticHeap,Verifier};
use tiny_fl::circuit::{Circuit,Outcome,SmtLibCircuit,SmtSolver};

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
                .arg(Arg::new("solver-path").long("solver-path").default_value("z3").value_parser(value_parser!(OsString)))
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
    let solver_path = args.get_one::<OsString>("solver-path").unwrap();
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
        // Statically linked Z3 has been requested.
        z3_check(&parser.heap,&terms)
    } else {
        // Construcnt SmtSolver instance
        let solver = SmtSolver::new(solver_path.as_ref());
        // Construct SmtLib circuit
        let smtlib = SmtLibCircuit::new(solver);
        // Do it!
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
