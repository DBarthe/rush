
pub enum Token {
    Word(~str),
    Operator(~str),
    IoNumber(~str),
    NewLine,
    EOF,
}
    
pub struct Lexer {    
    data: ~str,
    index: uint,
}

impl Lexer {

    /// Instantiate a Lexer strucure with a text data.
    pub fn new(data: ~str) -> Lexer {
        Lexer {
            data: data,
            index: 0,
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
}

impl Iterator<Token> for Lexer {

    /// Scan the data and return the next token.
    fn next(&mut self) -> Option<Token> {

        let current_chars = StrBuf::new();        
        let quoted: Option<char> = None;
        let excpected_token: Option<Token> = None; 

        match self.peek_one() {
            _ => {},
            // if eof => delimit current token or return EOF
            // if not quoted and previous is part of operator
                // if current can be join to form longer operator => add it
                // else => delimit
            // if not quoted and is a quoting chars (',",\) => set quoted, add it. token become word, fetch until the ending quote inclus. If newline => delimit
            // if not quot first subsitution ($, `) => special recursive function
            // if not quot begining of operator => delimit or start operator
            // if not quot \n => delimit
            // if not quot blank => discard or delimit
            // if prev is word => append it
            // if # => discard command until \n not inclus
            // else => new word
        };
        None
    }
}