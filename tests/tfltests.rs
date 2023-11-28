use std::fs;
use std::path::PathBuf;
use tiny_fl::{Parser,RustPrinter};

pub static REFTESTS_DIR: &str = "tests/files";

// Include the programmatically generated test file.
include!(concat!(env!("OUT_DIR"), "/tfltests.rs"));

/// Run a specific test by loading the file out of the reference tests
/// repository and attempting to parse it.  All reference tests should
/// parse correctly.
fn check(test: &str) {
    // Construct filename
    let mut path = PathBuf::from(REFTESTS_DIR);
    path.push(test);
    let filename = path.as_path().to_str().unwrap();
    // Read the test file
    let input = fs::read_to_string(filename).unwrap();
    // Parse it
    let mut parser = Parser::new(&input);
    // Check it
    let terms = match parser.parse() {
        Ok(terms) => terms,
        Err(_) => {
            panic!("failed parsing: {}",filename);
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
}
