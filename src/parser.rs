use crate::{BinOp,EOF,Lexer,Function,Term,Token,TokenType,SyntacticHeap};

/// Defines the set of tokens which are considered to identify logical
/// connectives (e.g. `&&`, `||`, etc).
pub const LOGICAL_CONNECTIVES : &[TokenType] = &[
    TokenType::AmpersandAmpersand,
    TokenType::BarBar
];

/// Defines the set of tokens which are considered to identify
/// arithmetic comparators (e.g. `<`, `<=`, `==`, etc).
pub const ARITHMETIC_COMPARATORS : &[TokenType] = &[
    TokenType::EqualsEquals,
    TokenType::ShreakEquals,
    TokenType::LeftAngle,
    TokenType::LeftAngleEquals,
    TokenType::RightAngle,
    TokenType::RightAngleEquals
];

/// Defines the set of tokens which are considered to identify
/// arithmetic operators (e.g. `+`, `-`, `*`, etc).
pub const ARITHMETIC_OPERATORS : &[TokenType] = &[
    TokenType::Minus,
    TokenType::Percent,
    TokenType::Plus,
    TokenType::RightSlash,
    TokenType::Star
];

pub const BINARY_CONNECTIVES : &[ &[TokenType] ] = &[
    ARITHMETIC_OPERATORS,
    ARITHMETIC_COMPARATORS,
    LOGICAL_CONNECTIVES
];

// ===================================================================
// Parser
// ===================================================================

/// Simplest possible parser.  Its a combination lexer and parser!
pub struct Parser {
    /// Character sequence being parsed
    lexer: Lexer,
    /// Heap being constructed
    pub heap: SyntacticHeap
}

impl Parser {
    /// Construct a parser from a string slice.
    pub fn new(content: &str) -> Self {
        // Convert string slice into Vec<char>
        let lexer = Lexer::new(content);
        // Create fresh heap
        let heap = SyntacticHeap::new();
        // Done
        Self{lexer, heap}
    }

    /// Parse a line of text into a term.
    pub fn parse(&mut self) -> Result<Vec<usize>,()> {
        let mut terms = Vec::new();
        //
        while self.lexer.lookahead(0) != EOF {
            terms.push(self.parse_declaration()?);
        }
        //
        Ok(terms)
    }

    fn parse_declaration(&mut self) -> Result<usize,()> {
        let lookahead = self.lexer.lookahead(0);
        //
        match lookahead.kind {
            TokenType::Function => self.parse_decl_function(),
            _ => {
                Err(())
            }
        }
    }

    // ===============================================================
    // Declarations
    // ===============================================================

    fn parse_decl_function(&mut self) -> Result<usize,()> {
        self.lexer.expect(TokenType::Function);
        // Parse function name
        let id = self.lexer.expect(TokenType::Identifier);
        // Parse declared parameters
        let params = self.parse_decl_params()?;
        // Parse optional return
        let rets = if self.lexer.matches(TokenType::RightArrow) {
	    self.parse_decl_params()?
        } else {
            Vec::new()
        };
        let requires = self.parse_decl_requires()?;
        let ensures = self.parse_decl_ensures()?;
        // Parse function body
        let body = self.parse_block()?;
        // Done
        let name = self.lexer.to_string(&id);
        let fun = Function{name,params,rets,requires,ensures,body};
        Ok(self.heap.alloc(Term::Function(fun)))
    }

    fn parse_decl_params(&mut self) -> Result<Vec<(usize,String)>,()> {
        let mut params = Vec::new();
        self.lexer.expect(TokenType::LeftBrace);
        let mut lookahead = self.lexer.lookahead(0);
        //
        while lookahead.kind != TokenType::RightBrace {
            if !params.is_empty() { self.lexer.expect(TokenType::Comma); }
            let t = self.parse_type()?;
            let var = self.parse_identifier()?;
            params.push((t,var));
            lookahead = self.lexer.lookahead(0);
        }
        // Done
        self.lexer.expect(TokenType::RightBrace);
        Ok(params)
    }

    fn parse_decl_requires(&mut self) -> Result<Vec<usize>,()> {
        let mut requires = Vec::new();
        while self.lexer.matches(TokenType::Requires) {
            requires.push(self.parse_expr()?);
        }
        Ok(requires)
    }

    fn parse_decl_ensures(&mut self) -> Result<Vec<usize>,()> {
        // TODO: include return values
        let mut ensures = Vec::new();
        while self.lexer.matches(TokenType::Ensures) {
            ensures.push(self.parse_expr()?);
        }
        Ok(ensures)
    }

    // ===============================================================
    // Statement Blocks
    // ===============================================================

    /// Parse a _block_, which is a term wrapped in curly braces
    /// (e.g. `{ [] }`).
    fn parse_block(&mut self) -> Result<usize,()> {
        let mut terms = Vec::new();
        // Blocks begin with open curly brace
        self.lexer.expect(TokenType::LeftCurly);
        // Keep going until closing curly brace
        while self.lexer.lookahead(0).kind != TokenType::RightCurly {
            // Parse separator (if applicable)
            if !terms.is_empty() { self.lexer.expect(TokenType::SemiColon); }
            // Parse statement or terminating expression
            match self.lexer.lookahead(0).kind {
                TokenType::Assert => {
                    terms.push(self.parse_stmt_assert()?);
                }
                TokenType::Assume => {
                    terms.push(self.parse_stmt_assume()?);
                }
                TokenType::RightCurly => {
                    // No trailing expression
                }
                _ => {
                    // Trailing expression
                    terms.push(self.parse_expr()?);
                }
            }
        }
        //
        self.lexer.expect(TokenType::RightCurly);
        Ok(self.heap.alloc(Term::Block(terms)))
    }

    fn parse_stmt_assert(&mut self) -> Result<usize,()> {
        self.lexer.expect(TokenType::Assert);
        let expr = self.parse_expr()?;
        Ok(self.heap.alloc(Term::Assert(expr)))
    }

    fn parse_stmt_assume(&mut self) -> Result<usize,()> {
        self.lexer.expect(TokenType::Assume);
        let expr = self.parse_expr()?;
        Ok(self.heap.alloc(Term::Assume(expr)))
    }

    // ===============================================================
    // Expressions
    // ===============================================================

    pub fn parse_expr(&mut self) -> Result<usize,()> {
        self.parse_expr_binary(3)
    }

    /// Parse a binary expression at a given _level_.  Higher levels
    /// indicate expressions which bind _less tightly_.  Furthermore,
    /// level `0` corresponds simply to parsing a unary expression.
    fn parse_expr_binary(&mut self, level: usize) -> Result<usize,()> {
        if level == 0 {
            self.parse_expr_postfix()
        } else {
            let tokens = BINARY_CONNECTIVES[level-1];
            // Parse level below
    	    let lhs = self.parse_expr_binary(level-1)?;
            // Check whether binary connective follows
            match self.lexer.match_any(tokens) {
                Some(t) => {
                    // FIXME: turn this into a loop!
	            let rhs = self.parse_expr_binary(level-1)?;
                    // NOTE: following is safe because can only match
                    // tokens which will be accepted.
                    let bop = Self::binop_from_token(t.kind).unwrap();
                    // Done
                    Ok(self.heap.alloc(Term::Binary(bop,lhs,rhs)))
                }
                None => Ok(lhs)
            }
        }
    }

    fn parse_expr_postfix(&mut self) -> Result<usize,()> {
        // Parse the source term
        let mut src = self.parse_expr_unit()?;
        //
        let mut lookahead = self.lexer.lookahead(0);
        // Attempt to parse a postfix operator
        while Self::is_postfix_operator(lookahead) {
            match lookahead.kind {
                TokenType::LeftSquare => {
                    src = self.parse_expr_arrayaccess(src)?;
                }
                TokenType::Dot => {
                    src = self.parse_expr_tupleaccess(src)?;
                }
                _ => {}
            }
            // Continue
            lookahead = self.lexer.lookahead(0);
        }
        // Done
        Ok(src)
    }

    fn parse_expr_unit(&mut self) -> Result<usize,()> {
        let lookahead = self.lexer.lookahead(0);
        //
        match lookahead.kind {
            TokenType::Bar => self.parse_expr_arraylength(),
            TokenType::BoolLiteral(v) => self.parse_literal_bool(v),
            TokenType::LeftBrace => self.parse_expr_braced(),
            TokenType::LeftSquare => self.parse_expr_arrayconstructor(),
            TokenType::Identifier => {
                // Disambiguate static invocation from variable access
                if self.lexer.lookahead(1).kind == TokenType::LeftBrace {
                    self.parse_expr_staticinvoke()
                } else {
                    self.parse_expr_varaccess()
                }
            }
            TokenType::IntLiteral => self.parse_literal_int(),
            TokenType::If => self.parse_expr_ifelse(),
            _ => {
                panic!("unexpected token {lookahead:?}");
            }
        }
    }

    fn parse_expr_arrayaccess(&mut self, mut src: usize) -> Result<usize,()> {
        self.lexer.expect(TokenType::LeftSquare);
        let index = self.parse_expr_unit()?;
        // Check whether access or slice
        if self.lexer.lookahead(0).kind == TokenType::DotDot {
            self.lexer.expect(TokenType::DotDot);
            let end = self.parse_expr_unit()?;
            self.lexer.expect(TokenType::RightSquare);
            // Allocate access expression
            src = self.heap.alloc(Term::ArraySlice{src,start:index,end});
        } else {
            self.lexer.expect(TokenType::RightSquare);
            // Allocate access expression
            src = self.heap.alloc(Term::ArrayAccess{src,index});
        }
        Ok(src)
    }

    fn parse_expr_arrayconstructor(&mut self) -> Result<usize,()> {
        let mut terms = Vec::new();
        // Parse left square brace
        self.lexer.expect(TokenType::LeftSquare);
        //
        if self.lexer.lookahead(0).kind != TokenType::RightSquare {
            let e1 = self.parse_expr()?;
            // Decide between literal and generator
            if self.lexer.lookahead(0).kind == TokenType::SemiColon {
                self.lexer.expect(TokenType::SemiColon);
                let e2 = self.parse_expr()?;
                self.lexer.expect(TokenType::RightSquare);
                //
                return Ok(self.heap.alloc(Term::ArrayGenerator(e1,e2)));
            } else {
                terms.push(e1);
                // Parse remainder
                while self.lexer.lookahead(0).kind != TokenType::RightSquare {
                    self.lexer.expect(TokenType::Comma);
                    terms.push(self.parse_expr()?);
                }
            }
        }
        // Match right square brace
        self.lexer.expect(TokenType::RightSquare);
        //
        Ok(self.heap.alloc(Term::ArrayConstructor(terms)))
    }

    fn parse_expr_arraylength(&mut self) -> Result<usize,()> {
        self.lexer.expect(TokenType::Bar);
        // Parse source expression
        let src = self.parse_expr()?;
        //
        self.lexer.expect(TokenType::Bar);
        // Done
        Ok(self.heap.alloc(Term::ArrayLength(src)))
    }

    fn parse_expr_braced(&mut self) -> Result<usize,()> {
        // Parse opening bracket
        self.lexer.expect(TokenType::LeftBrace);
        // Parse comma-separated terms
        let terms = self.parse_exprs_until(TokenType::RightBrace)?;
        // Parse right brace
        self.lexer.expect(TokenType::RightBrace);
        //
        if terms.len() == 1 {
            // Normal braced expression?
            Ok(self.heap.alloc(Term::Braced(terms[0])))
        } else {
            Ok(self.heap.alloc(Term::TupleConstructor(terms)))
        }
    }

    fn parse_expr_ifelse(&mut self) -> Result<usize,()> {
        self.lexer.expect(TokenType::If);
        // Parse condition
        let cond = self.parse_expr()?;
        // Parse true branch
        let tt = self.parse_block()?;
        // Parse false branch (currently required)
        self.lexer.expect(TokenType::Else);
        let ff = self.parse_block()?;
        // Done
        Ok(self.heap.alloc(Term::IfElse{cond,tt,ff}))
    }

    fn parse_expr_staticinvoke(&mut self) -> Result<usize,()> {
        let id = self.lexer.expect(TokenType::Identifier);
        let name = self.lexer.to_string(&id);
        // Parse left brace
        self.lexer.expect(TokenType::LeftBrace);
        // Parse terms within literal
        let terms = self.parse_exprs_until(TokenType::RightBrace)?;
        // Match right brace
        self.lexer.expect(TokenType::RightBrace);
        //
        Ok(self.heap.alloc(Term::StaticInvoke(name,terms)))
    }

    fn parse_expr_tupleaccess(&mut self, src: usize) -> Result<usize,()> {
        self.lexer.expect(TokenType::Dot);
        let tok = self.lexer.expect(TokenType::IntLiteral);
        let s = self.lexer.to_string(&tok);
        let i = s.parse::<usize>().unwrap();
        Ok(self.heap.alloc(Term::TupleAccess(src,i)))
    }

    fn parse_expr_varaccess(&mut self) -> Result<usize,()> {
        let id = self.lexer.expect(TokenType::Identifier);
        let name = self.lexer.to_string(&id);
        // Parse as variable access
        Ok(self.heap.alloc(Term::VarAccess(name)))
    }

    /// Parse a sequence of zero or more comma-separated terms until a
    /// given end token is encountered.
    fn parse_exprs_until(&mut self, end: TokenType) -> Result<Vec<usize>,()> {
        let mut terms = Vec::new();
        //
        let mut lookahead = self.lexer.lookahead(0);
        while lookahead.kind != end {
            if !terms.is_empty() {
                self.lexer.expect(TokenType::Comma);
            }
            let ith = self.parse_expr()?;
            terms.push(ith);
            lookahead = self.lexer.lookahead(0);
        }
        //
        Ok(terms)
    }

    // ===============================================================
    // Literals
    // ===============================================================

    fn parse_literal_bool(&mut self, val: bool) -> Result<usize,()> {
        self.lexer.expect(TokenType::BoolLiteral(val));
        Ok(self.heap.alloc(Term::BoolLiteral(val)))
    }

    fn parse_literal_int(&mut self) -> Result<usize,()> {
        let tok = self.lexer.expect(TokenType::IntLiteral);
        let s = self.lexer.to_string(&tok);
        let i = s.parse::<usize>().unwrap();
        Ok(self.heap.alloc(Term::IntLiteral(i)))
    }

    // ===============================================================
    // Types
    // ===============================================================

    fn parse_type(&mut self) -> Result<usize,()> {
        let mut src = self.parse_unit_type()?;
        let mut lookahead = self.lexer.lookahead(0);
        // Parse array type
        while lookahead.kind == TokenType::LeftSquare {
            self.lexer.expect(TokenType::LeftSquare);
            self.lexer.expect(TokenType::RightSquare);
            // Allocate access expression
            src = self.heap.alloc(Term::ArrayType(src));
            lookahead = self.lexer.lookahead(0);
        }
        //
        Ok(src)
    }

    fn parse_unit_type(&mut self) -> Result<usize,()> {
        let lookahead = self.lexer.lookahead(0);
        //
        match lookahead.kind {
            TokenType::Uint => self.parse_uint_type(),
            TokenType::Bool => self.parse_bool_type(),
            TokenType::LeftBrace => self.parse_tuple_type(),
            _ => {
                panic!("unexpected token {lookahead:?}");
            }
        }
    }

    fn parse_bool_type(&mut self) -> Result<usize,()> {
        self.lexer.expect(TokenType::Bool);
        Ok(self.heap.alloc(Term::BoolType))
    }

    fn parse_uint_type(&mut self) -> Result<usize,()> {
        self.lexer.expect(TokenType::Uint);
        Ok(self.heap.alloc(Term::IntType(false)))
    }

    fn parse_tuple_type(&mut self) -> Result<usize,()> {
        let mut types = Vec::new();
        self.lexer.expect(TokenType::LeftBrace);
        while self.lexer.lookahead(0).kind != TokenType::RightBrace {
            if !types.is_empty() {
                self.lexer.expect(TokenType::Comma);
            }
            types.push(self.parse_type()?);
        }
        self.lexer.expect(TokenType::RightBrace);
        Ok(self.heap.alloc(Term::TupleType(types)))
    }

    // ===============================================================
    // Misc
    // ===============================================================

    fn parse_identifier(&mut self) -> Result<String,()> {
        let ith = self.lexer.expect(TokenType::Identifier);
        Ok(self.lexer.to_string(&ith))
    }

    fn is_postfix_operator(token: Token) -> bool {
        token.kind == TokenType::LeftSquare || token.kind == TokenType::Dot
    }

    /// Construct a `BinOp` from a `TokenType`.
    fn binop_from_token(token: TokenType) -> Option<BinOp> {
	let bop = match token {
            // Equality
            TokenType::EqualsEquals => BinOp::Equals,
            TokenType::ShreakEquals => BinOp::NotEquals,
            // Comparison
	    TokenType::LeftAngle => BinOp::LessThan,
            TokenType::LeftAngleEquals => BinOp::LessThanOrEquals,
            TokenType::RightAngle => BinOp::GreaterThan,
            TokenType::RightAngleEquals => BinOp::GreaterThanOrEquals,
            // Arithmetic
            TokenType::Minus => BinOp::Subtract,
	    TokenType::Percent => BinOp::Remainder,
	    TokenType::Plus => BinOp::Add,
            TokenType::RightSlash => BinOp::Divide,
            TokenType::Star => BinOp::Multiply,
            // Logical
            TokenType::AmpersandAmpersand => BinOp::LogicalAnd,
            TokenType::BarBar => BinOp::LogicalOr,
            // No match
	    _ => { unreachable!(); }
	};
        Some(bop)
    }

}
