// ===================================================================
// Token
// ===================================================================

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum TokenType {
    AmpersandAmpersand,
    Assert,
    Assume,
    Bar,
    BarBar,
    Bool,
    BoolLiteral(bool),
    Comma,
    Dot,
    DotDot,
    EqualsEquals,
    Else,
    EOF,
    Ensures,
    Function,
    Identifier,
    If,
    IntLiteral,
    LeftAngle,
    LeftAngleEquals,
    LeftBrace,
    LeftCurly,
    LeftSquare,
    Minus,
    Percent,
    Plus,
    PlusPlus,
    Requires,
    RightAngle,
    RightAngleEquals,
    RightArrow,
    RightBrace,
    RightCurly,
    RightSlash,
    RightSquare,
    SemiColon,
    ShreakEquals,
    Star,
    Uint,
    WhiteSpace
}

/// Represents a single token generated from a string slice.  This
/// identifies where the token starts and ends in the original slice.
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Token {
    /// Type of the token
    pub kind : TokenType,
    pub offset: usize,
    pub len: usize
}

impl Token {
    pub fn relocate(&mut self, offset: usize) {
        self.offset = offset;
    }
}

/// Represents the end of the input stream.  This is helpful because
/// it allows us to avoid using `Option<>` everywhere.
pub const EOF : Token = Token{kind: TokenType::EOF,offset:0,len:0};

// ===================================================================
// Keywords
// ===================================================================

const ASSERT : &[char] = &['a','s','s','e','r','t'];
const ASSUME : &[char] = &['a','s','s','u','m','e'];
const BOOL : &[char] = &['b','o','o','l'];
const ELSE : &[char] = &['e','l','s','e'];
const ENSURES : &[char] = &['e','n','s','u','r','e','s'];
const FALSE : &[char] = &['f','a','l','s','e'];
const FUNCTION : &[char] = &['f','u','n','c','t','i','o','n'];
const IF : &[char] = &['i','f'];
const REQUIRES : &[char] = &['r','e','q','u','i','r','e','s'];
const TRUE : &[char] = &['t','r','u','e'];
const UINT : &[char] = &['u','i','n','t'];

// ===================================================================
// Scanners
// ===================================================================

/// Scan for next token
fn scan(input: &[char]) -> Token {
    if let Ok(tok) = scan_whitespace(input) { return tok; }
    if let Ok(tok) = scan_double_operators(input) { return tok; }
    if let Ok(tok) = scan_single_operators(input) { return tok; }
    if let Ok(tok) = scan_int_literal(input) { return tok; }
    if let Ok(tok) = scan_keyword(input) { return tok; }
    if let Ok(tok) = scan_identifier(input) { return tok; }
    // TODO: better error handling :)
    panic!("unexpected character: {}", input[0]);
}

/// Scan all single-character operators.
fn scan_single_operators(input: &[char]) -> Result<Token,()> {
    if input.is_empty() {
        Err(())
    } else {
        let t = match input[0] {
            '|' => TokenType::Bar,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '<' => TokenType::LeftAngle,
            '(' => TokenType::LeftBrace,
            '{' => TokenType::LeftCurly,
            '[' => TokenType::LeftSquare,
            '-' => TokenType::Minus,
            '%' => TokenType::Percent,
            '+' => TokenType::Plus,
            '>' => TokenType::RightAngle,
            ')' => TokenType::RightBrace,
            '}' => TokenType::RightCurly,
            '/' => TokenType::RightSlash,
            ']' => TokenType::RightSquare,
            '*' => TokenType::Star,
            ';' => TokenType::SemiColon,
            _ => { return Err(()); }
        };
        //
        Ok(Token{kind:t, offset:0, len:1})
    }
}

/// Scan all double-character operators.
fn scan_double_operators(input: &[char]) -> Result<Token,()> {
    if input.len() <= 1 {
        Err(())
    } else {
        let t = match (input[0], input[1]) {
            ('&','&') => TokenType::AmpersandAmpersand,
            ('|','|') => TokenType::BarBar,
            ('=','=') => TokenType::EqualsEquals,
            ('.','.') => TokenType::DotDot,
            ('<','=') => TokenType::LeftAngleEquals,
            ('+','+') => TokenType::PlusPlus,
            ('-','>') => TokenType::RightArrow,
            ('>','=') => TokenType::RightAngleEquals,
            ('!','=') => TokenType::ShreakEquals,
            _ => { return Err(()); }
        };
        //
        Ok(Token{kind:t, offset:0, len:2})
    }
}

/// Scan an integer literal which is a sequence of zero or more
/// digits.
fn scan_int_literal(input: &[char]) -> Result<Token,()> {
    let mut i = 0;
    // Continue matching
    while i < input.len() && input[i].is_ascii_digit() {
        i += 1;
    }
    // Check what happened
    if i != 0 {
        // Something matched
        Ok(Token{kind: TokenType::IntLiteral, offset: 0, len: i})
    } else {
        Err(())
    }
}


/// Scan an identifier which starts with an alpabetic character, or an
/// underscore and subsequently contains zero or more alpha-number
/// characters or underscores.
fn scan_identifier(input: &[char]) -> Result<Token,()> {
    if !input.is_empty() && is_identifier_start(input[0]) {
        let mut i = 0;
        // Continue matching
        while i < input.len() && is_identifier_middle(input[i]) {
            i += 1;
        }
        // Check what happened
        if i != 0 {
            // Something matched
            return Ok(Token{kind: TokenType::Identifier, offset: 0, len: i});
        }
    }
    Err(())
}

/// Scan a keyword, which is a special form of identifier.
fn scan_keyword(input: &[char]) -> Result<Token,()> {
    // Attempt to scan identifier
    let ident = scan_identifier(input)?;
    // Check whether matches a keyword
    let t = match &input[0..ident.len] {
        ASSERT => TokenType::Assert,
        ASSUME => TokenType::Assume,
        BOOL => TokenType::Bool,
        ELSE => TokenType::Else,
        ENSURES => TokenType::Ensures,
        FALSE => TokenType::BoolLiteral(false),
        IF => TokenType::If,
        FUNCTION => TokenType::Function,
        REQUIRES => TokenType::Requires,
        TRUE => TokenType::BoolLiteral(true),
        UINT => TokenType::Uint,
        _ => { return Err(()); }
    };
    // Success!
    Ok(Token{kind:t,offset:0,len:ident.len})
}

fn scan_whitespace(input: &[char]) -> Result<Token,()> {
    let mut i = 0;
    // Continue matching
    while i < input.len() && is_whitespace(input[i]) {
        i += 1;
    }
    // See what happened
    if i >= 1 {
        Ok(Token{kind: TokenType::WhiteSpace, offset: 0, len: i})
    } else {
        Err(())
    }
}

/// Determine whether a given character is the start of an identifier.
fn is_identifier_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

/// Determine whether a given character can occur in the middle of an
/// identifier
fn is_identifier_middle(c: char) -> bool {
    c.is_ascii_digit() || is_identifier_start(c)
}

/// Determine whether a given character is considered _whitespace_ or
/// not.
fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\n'
}

// ===================================================================
// Lexer
// ===================================================================

pub struct Lexer {
    /// Character sequence being parsed
    chars: Vec<char>,
    /// Index into char sequence.
    offset: usize
}

impl Lexer {
    /// Construct a parser from a string slice.
    pub fn new(content: &str) -> Self {
        // Convert string slice into Vec<char>
        let chars = content.chars().collect();
        // Done
        Self{chars, offset: 0}
    }

    pub fn lookahead(&mut self, mut n: usize) -> Token {
        let mut tok = Self::lookahead_from(&self.chars,self.offset);
        //
        while n > 0 {
            tok = Self::lookahead_from(&self.chars,tok.offset + tok.len);
            n -= 1;
        }
        //
        tok
    }

    pub fn matches(&mut self, kind: TokenType) -> bool {
        let lookahead = self.lookahead(0);
        //
        if lookahead.kind == kind {
            self.accept(&lookahead);
            true
        } else {
            false
        }
    }

    pub fn match_any(&mut self, tokens: &[TokenType]) -> Option<Token> {
        let lookahead = self.lookahead(0);
        //
        for t in tokens {
            if  lookahead.kind == *t {
                return Some(self.expect(*t))
            }
        }
        None
    }

    /// Construct a string from a span of the character array
    /// determined by a token.
    pub fn to_string(&self, token: &Token) -> String {
        let n = token.offset;
        let slice = &self.chars[n..n+token.len];
        slice.iter().collect()
    }

    pub fn accept(&mut self, token: &Token) {
        // Skip whitespace (this could be made more efficient)
        self.skip_whitespace();
        // sanity check
        assert!(self.lookahead(0) == *token);
        // for now
        self.offset += token.len
    }

    pub fn expect(&mut self, kind: TokenType) -> Token {
        let tok = self.lookahead(0);
        if tok.kind != kind {
            panic!("expected {kind:?} found {tok:?}");
        }
        self.accept(&tok);
        tok
    }

    // ===============================================================

    fn skip_whitespace(&mut self) {
        if self.offset < self.chars.len() {
            // Scan operators
            let tok = scan(&self.chars[self.offset..]);
            // See what we got
            if tok.kind == TokenType::WhiteSpace {
                self.offset += tok.len;
            }
        }
    }

    fn lookahead_from(chars: &[char], offset: usize) -> Token {
        if offset >= chars.len() {
            EOF
        } else {
            // Create slice
            let slice = &chars[offset..];
            // Scan operators
            let mut tok = scan(slice);
            //
            if tok.kind == TokenType::WhiteSpace {
                // Continue
                Self::lookahead_from(chars, offset + tok.len)
            } else {
                // Set its offset
                tok.relocate(offset);
                // Done
                tok
            }
        }
    }

}
