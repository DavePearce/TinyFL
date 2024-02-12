use std::io::{Result,Write};

use super::ast::*;

pub struct SmtLibWriter<T:Write> {
    /// Output writer
    out: T
}

impl<T:Write> SmtLibWriter<T> {
    pub fn new(out: T) -> Self {
        Self{out}
    }

    pub fn write(&mut self, commands: &[Command]) -> Result<()> {
        for cmd in commands {
            self.write_command(cmd)?;
        }
        Ok(())
    }

    fn write_command(&mut self, cmd: &Command) -> Result<()> {
        match cmd {
            Command::Assert(expr) => self.write_assert(expr),
            Command::CheckSat => self.write_checksat()
        }
    }

    fn write_checksat(&mut self) -> Result<()> {
        writeln!(self.out,"(check-sat)")
    }

    fn write_assert(&mut self, expr: &Expr) -> Result<()> {
        write!(self.out,"(assert ")?;
        self.write_expr(expr)?;
        writeln!(self.out,")")
    }

    fn write_expr(&mut self, expr: &Expr) -> Result<()> {
        match expr {
            Expr::Integer(i) => { write!(self.out,"{i}") }
            Expr::Boolean(b) => { write!(self.out,"{b}") }
            Expr::VarAccess(n) => { write!(self.out,"{n}") }
            Expr::Nary(op,args) => self.write_nary(op,args)
        }
    }

    fn write_nary(&mut self, op: &Op, args: &[Expr]) -> Result<()> {
        write!(self.out,"({}",op.as_str())?;
        for arg in args {
            write!(self.out," ")?;
            self.write_expr(arg)?;
        }
        write!(self.out,")")
    }
}
