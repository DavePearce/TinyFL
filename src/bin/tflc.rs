use std::error::Error;
use std::{fs};
use clap::{arg, Arg, ArgMatches, Command};
use tiny_fl::{Parser,RustPrinter};

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
    // Construct verifier instance
    // let vcg = VcGenerator::new(&parser.heap);
    // // Extract verification condition program
    // let vcp = vcg.generate_all(&terms);
    // // Run the program
    // let errors = vcp.check();
    // for err in errors {
    //     match err {
    //         BoundedResult::Ok(_) => {}
    //         BoundedResult::Err(_) => { panic!("verification failure"); }
    //         BoundedResult::OutOfResource => { panic!("verification out-of-resource"); }	    
    //     }
    // }
    // //
    // Ok(true)
    todo!("Verification needs to be implemented!");
}
