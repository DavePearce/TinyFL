use crate::{BinOp,Function,SyntacticHeap,Term};

/// Simplest possible code generator
pub struct RustPrinter<'a> {
    heap: &'a SyntacticHeap,
    out: String,
    indent: usize
}

impl<'a> RustPrinter<'a> {
    /// Create a new printer
    pub fn new(heap: &'a SyntacticHeap) -> Self {
        let out = Self::preamble().to_string();
        let indent = 0;
        Self{heap, out, indent}
    }

    pub fn preamble() -> &'static str {
        ""
    }

    pub fn write(&mut self, text: &str) {
        self.out.push_str(text);
    }

    pub fn writeln(&mut self, text: &str) {
        self.write(text);
        self.out.push_str("\n");
    }

    pub fn indent(&mut self, text: &str) {
        for _i in 0..self.indent {
            self.out.push_str("   ");
        }
        self.write(text);
    }

    pub fn done(self) -> String {
        self.out
    }

    /// Print out a program!
    pub fn generate(&mut self, index: usize) {
        // Must be valid term
        assert!(index < self.heap.len());
        //
        let term = self.heap.get(index);
        match term {
            // Declarations
            Term::Function(fun) => self.gen_function(fun),
            // Statements
            Term::Assert(src) => self.gen_assert(*src),
            Term::Assume(src) => self.gen_assert(*src),
            Term::Block(terms) => self.gen_block(terms),
            // Expressions
            Term::ArrayAccess{src,index} => self.gen_array_access(*src,*index),
            Term::ArrayGenerator(item,len) => self.gen_array_generator(*item,*len),
            Term::ArraySlice{src,start,end} => self.gen_array_slice(*src,*start,*end),
            Term::ArrayLength(src) => self.gen_array_length(*src),
            Term::ArrayConstructor(vs) => self.gen_array_constructor(vs),
            Term::Binary(bop,l,r) => self.gen_binary(*bop,*l,*r),
            Term::BoolLiteral(v) => self.gen_bool_literal(*v),
            Term::Braced(v) => self.gen_braced(*v),
            Term::IfElse{cond,tt,ff} => self.gen_if(*cond,*tt,*ff),
            Term::IntLiteral(v) => self.gen_int_literal(*v),
            Term::StaticInvoke(name,args) => self.gen_static_invoke(name,args),
            Term::TupleAccess(src,index) => self.gen_tuple_access(*src,*index),
            Term::TupleConstructor(vs) => self.gen_tuple_constructor(vs),
            Term::VarAccess(v) => self.gen_var_access(v),
            // Types
            Term::ArrayType(src) => self.gen_array_type(*src),
            Term::TupleType(types) => self.gen_tuple_type(types),
            Term::IntType(s) => self.gen_int_type(*s),
            Term::BoolType => self.gen_bool_type()
        }
    }

    // ===============================================================
    // Declarations
    // ===============================================================

    fn gen_function(&mut self, fun: &Function) {
        self.write("fn ");
        self.write(&fun.name);
        self.write("(");
        for i in 0..fun.params.len() {
            let (t,v) = &fun.params[i];
            if i != 0 { self.write(", "); }
            self.write(v);
            self.write(": ");
            self.generate(*t);
        }
        self.write(")");
	if !fun.rets.is_empty() {
            self.write(" -> ");
	    self.write("(");
            for i in 0..fun.rets.len() {
		let (t,v) = &fun.rets[i];
		if i != 0 { self.write(", "); }
		self.write(v);
		self.write(": ");
		self.generate(*t);
            }
            self.write(")");	    
        }
        self.generate(fun.body);
        self.writeln("");
    }

    // ===============================================================
    // Statements
    // ===============================================================

    fn gen_assert(&mut self, src: usize) {
        self.write("assert!(");
        self.generate(src);
        self.write(")");
    }

    fn gen_block(&mut self, terms: &Vec<usize>) {
        self.writeln(" {");
        self.indent += 1;
        self.indent("");
        for i in 0..terms.len() {
            if i > 0 { self.writeln("; "); self.indent(""); }
            self.generate(terms[i]);
        }
        self.writeln("");
        self.indent -= 1;
        self.indent("}");
    }

    // ===============================================================
    // Expressions
    // ===============================================================

    fn gen_array_access(&mut self, src: usize, index: usize) {
        self.generate(src);
        self.write("[");
        self.generate(index);
        self.write("]");
    }

    fn gen_array_constructor(&mut self, terms: &Vec<usize>) {
        self.write("vec![");
        for i in 0..terms.len() {
            if i != 0 {
                self.write(",");
            }
            self.generate(terms[i]);
        }
        self.write("]");
    }

    fn gen_array_generator(&mut self, item: usize, len: usize) {
        self.write("vec![");
        self.generate(item);
        self.write(";");
        self.generate(len);
        self.write("]");
    }

    fn gen_array_length(&mut self, src: usize) {
        self.generate(src);
        self.write(".len()");
    }

    fn gen_array_slice(&mut self, src: usize, start: usize, end: usize) {
        self.generate(src);
        self.write("[");
        self.generate(start);
        self.write("..");
        self.generate(end);
        self.write("].to_vec()");
    }

    fn gen_binary(&mut self, bop: BinOp, lhs: usize, rhs: usize) {
        self.generate(lhs);
        self.write(Self::bop_to_str(bop));
        self.generate(rhs);
    }

    fn gen_braced(&mut self, src: usize) {
        self.write("(");
        self.generate(src);
        self.write(")");
    }

    fn gen_if(&mut self, cond: usize, tt: usize, ff: usize) {
        self.write("if ");
        self.generate(cond);
        self.generate(tt);
        self.write(" else");
        self.generate(ff);
    }

    fn gen_var_access(&mut self, var: &str) {
        self.write(var)
    }

    fn gen_static_invoke(&mut self, name: &String, args: &Vec<usize>) {
        self.write(name);
        self.write("(");
        for i in 0..args.len() {
            if i != 0 {
                self.write(",");
            }
            self.generate(args[i]);
        }
        self.write(")");
    }

    fn gen_tuple_access(&mut self, src: usize, index: usize) {
        self.generate(src);
        self.write(".");
        self.write(&index.to_string());
    }

    fn gen_tuple_constructor(&mut self, terms: &Vec<usize>) {
        self.write("(");
        for (i,_) in terms.iter().enumerate() {
            if i != 0 {
                self.write(",");
            }
            self.generate(terms[i]);
        }
        self.write(")");
    }

    // ===============================================================
    // Literals
    // ===============================================================

    fn gen_bool_literal(&mut self, val: bool) {
        if val {
            self.write("true");
        } else {
            self.write("false");
        }
    }

    fn gen_int_literal(&mut self, val: usize) {
        self.write(&val.to_string());
    }

    // ===============================================================
    // Types
    // ===============================================================

    fn gen_array_type(&mut self, src: usize) {
        self.write("Vec<");
        self.generate(src);
        self.write(">");
    }

    fn gen_bool_type(&mut self) {
        self.write("bool");
    }

    fn gen_int_type(&mut self, signed: bool) {
        if signed {
            self.write("isize");
        } else {
            self.write("usize");
        }
    }

    fn gen_tuple_type(&mut self, types: &Vec<usize>) {
        self.write("(");
        for (i,_) in types.iter().enumerate() {
            if i != 0 { self.write(","); }
            self.generate(types[i]);
        }
        self.write(")");
    }

    // Misc

    fn bop_to_str(bop: BinOp) -> &'static str {
        match bop {
            // Arithmetic
            BinOp::Add => "+",
            BinOp::Subtract => "-",
            BinOp::Divide => "/",
            BinOp::Multiply => "*",
            BinOp::Remainder => "%",
            // Comparators
            BinOp::Equals => "==",
            BinOp::NotEquals => "!=",
            BinOp::LessThan => "<",
            BinOp::LessThanOrEquals => "<=",
            BinOp::GreaterThan => ">",
            BinOp::GreaterThanOrEquals => ">=",
            // Logical
            BinOp::LogicalAnd => "&&",
            BinOp::LogicalImplies => todo!(),
            BinOp::LogicalOr => "||"
        }
    }
}
