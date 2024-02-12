use std::io;
use std::io::{BufWriter,Write};
use std::process::{Command,Stdio};
use crate::circuit;
use super::ast;
use super::SmtLibWriter;

pub enum SmtOutcome {
    Sat,
    Unsat,
    Maybe
}

pub fn smtsolver_exec(commands: &[ast::Command]) -> Vec<SmtOutcome> {
    let buffer = BufWriter::new(Vec::new());
    let bytes = SmtLibWriter::new(Vec::new()).write(commands).unwrap();
    let smt = String::from_utf8(bytes).unwrap();
    // Pipe to Child
    let mut child = Command::new("z3")
        .args(&["--smt2","--in"])
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to start SMT solver");
    // Grab the stdin handle.
    let mut stdin = child.stdin.take().unwrap();
    std::thread::spawn(move || {
        println!("{smt}");
        stdin.write_all(smt.as_bytes()).expect("Failed to write to stdin");
        stdin.write_all(b"(check-sat)").expect("Failed to write to stdin");
    });
    // Get output back
    let output = child.wait_with_output().expect("failed to read output");
    let sout = String::from_utf8_lossy(&output.stdout);
    //
    let mut outcomes = Vec::new();
    //
    for l in sout.lines() {
        match l {
            "sat" => outcomes.push(SmtOutcome::Sat),
            "unsat" => outcomes.push(SmtOutcome::Unsat),
            _ => outcomes.push(SmtOutcome::Maybe)
        }
    }
    //
    outcomes
}
