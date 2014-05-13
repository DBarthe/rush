
/// A token is stored via this enum which allow
/// certains types to keep the original lexeme.
#[deriving(Clone)]
pub enum Token {
    Word(~str),
    Operator(~str),
    IoNumber(~str),
    NewLine,
}

impl Token {

    /// Implement an easier way to test the token type
    /// (I don't know how to do this without a match statement)
    pub fn is_word(&self) -> bool {
        match *self {
            Word(_) => true,
            _ => false,
        }
    }
    pub fn is_operator(&self) -> bool {
        match *self {
            Operator(_) => true,
            _ => false,
        }
    }
    pub fn is_ionumber(&self) -> bool {
        match *self {
            IoNumber(_) => true,
            _ => false,
        }
    }
    pub fn is_newline(&self) -> bool {
        match *self {
            NewLine => true,
            _ => false,
        }
    }
}

/// During a call to Lexer::next(), this structure contains
/// informations about the next token to be returned.
struct LexerContext {   
    /// The current token like it has been found.
    current_chars: StrBuf,
    /// The current token is it currently quoted by a quote,
    /// double-quotes, or backslash ?
    quoted: Option<char>,
    /// The current determined token type, or None.
    expected_token: Option<Token>,
}

impl LexerContext {

    /// Create a new structure LexerContext.
    pub fn new() -> LexerContext {
        LexerContext {
            current_chars: StrBuf::new(),
            quoted: None,
            expected_token: None,
        }
    }

    /// Check if the current context is empty.
    pub fn empty(&self) -> bool {
        self.current_chars.len() == 0
    }
}


/// The lexer main structure.
pub struct Lexer {
    /// The raw input.
    data: ~str,
    /// The scanning position in the data.
    index: uint,
    /// Informations about the next token to be returned.
    context: LexerContext,
}

impl Lexer {

    /// Instantiate a Lexer strucure with a text data.
    pub fn new(data: ~str) -> Lexer {
        Lexer {
            data: data,
            index: 0,
            context: LexerContext::new(),
        }
    }

    /// Return the current character and move forward the index.
    fn consume_one(&mut self) -> char {
        let c = self.peek_one();
        if self.index < self.data.len() {
            self.index += 1;
        }
        return c;
    }

    /// Return the current character.
    fn peek_one(&mut self) -> char {
        if self.index >= self.data.len() {
            '\0'
        }
        else {
            self.data[self.index] as char
        }
    }

    /// Delimit the current token : make it with his correct lexeme and return it. 
    fn delimit_token(&mut self) -> Token {
        let lexeme = self.context.current_chars.to_owned();
        match self.context.expected_token {
            Some(ref token) => match *token {
                Word(_) => Word(lexeme),
                Operator(_) => Operator(lexeme),
                IoNumber(_) => IoNumber(lexeme),
                NewLine => NewLine,
            },
            None => Word(lexeme),
        }
    }
}

impl Iterator<Token> for Lexer {

    /// Scan the data and return the next token.
    fn next(&mut self) -> Option<Token> {

        // Cleanup the context.
        self.context = LexerContext::new();

        loop {
            let c = self.peek_one();

            // if eof => delimit current token or return EOF
            if c == '\0' {
                return match self.context.empty() {
                    true => None,
                    false => Some(self.delimit_token()),
                }
            }

            // check if current token is quoted and store the result in 'quoted'
            let quoted = self.context.quoted.is_none();

            // if previous is part of operator
            if !quoted && self.context.expected_token.is_some()
                && self.context.expected_token.as_ref().unwrap().is_operator() {
                    // if current can be join to form longer operator => add it
                    // else => delimit
            }

            // if is a quoting chars (',",\)
            else if !quoted && (c == '\'' || c == '"' || c == '\\') {
                // set quoted, add it. token become word, fetch until the ending quote inclus.
                self.context.quoted = Some(c);
                self.context.expected_token = Some(Word("".to_owned()));
                self.context.current_chars.push_char(c);
                self.consume_one();
            }

            else if !quoted && (c == '$' || c == '`') {
                // if first subsitution ($, `) => special recursive function
            }

            // if (not quoted begining of operator => delimit or start operator
            // test todo

            // if blank => discard or delimit
            else if !quoted && (c == ' ' ||  c == '\t') {
                match self.context.empty() {
                    true => self.consume_one(),
                    false => return Some(self.delimit_token()),
                };
            }

            // if \n => delimit or return new line
            else if c == '\n' {
                return match self.context.empty() {
                    true => Some(NewLine),
                    false => Some(self.delimit_token()),
                }
            }

            // if prev is word => append it
            else if self.context.expected_token.is_some()
                && self.context.expected_token.as_ref().unwrap().is_word() {
                    self.context.current_chars.push_char(c);
                    // if the current char is quoted and if the quote section
                    // is ending now => unset the quote status
                    if quoted && (self.context.quoted.unwrap() == '\\'
                                    || c == self.context.quoted.unwrap()) {
                        self.context.quoted = None;
                    }

            }

            // if # => discard command until \n not inclus
            else if c == '#' {
                assert!(self.context.expected_token.is_none());
                self.consume_one();
                loop {
                    let c = self.peek_one();
                    if c == '\n' && c == '\0' {
                        break;
                    }
                    self.consume_one();
                }
            }

            // else => new word
            else {
                assert!(self.context.expected_token.is_none());
                self.context.current_chars.push_char(c);
                self.context.expected_token = Some(Word("".to_owned()));
            }
        }
    }
}