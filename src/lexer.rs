pub enum TokenKind{
    Ident,
    OpenSeparator,
    CloseSeparator,
    Equal,
    Plus,
    Minus,
    Multiply,
    Divide,
    IfKeyword,
    ElseKeyword,
    LoopKeyword,
    Number,
    EndColon,

    Gimesi,
    Futtyentes
}

const FIXED_TOKENS: &(&char, TokenKind) = [
    (&'(', TokenKind::OpenSeparator),
    (&')', TokenKind::CloseSeparator),
    (&'=', TokenKind::Equal),
    (&'+', TokenKind::Plus),
    (&'-', TokenKind::Minus),
    (&'*', TokenKind::Multiply),
    (&'/', TokenKind::Divide)
];

/*const FIXED_TOKENS: &(&[char], TokenKinds) = &[
    (&['('], TokenKind::OpenSeparator),
    (&[')'], TokenKind::CloseSeparator),
    (&['='], TokenKind::Equal),
    (&['+'], TokenKind::Plus),
    (&['-'], TokenKind::Minus),
    (&['*'], TokenKind::Multiply),
    (&['/'], TokenKind::Divide)
];

const FIXED_KEYWORD_TOKENS : &(&[std::String], TokenKind) = &[
    (&["if"], TokenKind::IfKeyword),
    (&["else"], TokenKind::ElseKeyword),
    (&["loop"], TokenKind::LoopKeyword)
];*/

impl TokenKind{
    fn analyze(&self) ->&str{
        match self{
            Self::Ident => "identifier",
            Self::OpenSeparator => "open separator",
            Self::CloseSeparator => "close separator",
            Self::Equal => "equal",
            Self::Plus => "plus",
            Self::Minus => "minus",
            Self::Multiply => "multiply",
            Self::Divide => "divide",
            Self::IfKeyword => "if keyword",
            Self::ElseKeyword => "else keyword",
            Self::Number => "number",
            Self::LoopKeyword => "loop keyword",
            Self::EndColon => "end colon",
            _ => {report!("Hé ecsém, füttyent a kompájler")}
        }
    }
}

pub struct Token{
    kind : TokenKind,
    name : String
}

pub struct LexicalAnalyzer<'a>{
    content : Chars<'a>,        //src
    pub pos: usize,             //read pos, global pos
    pub read_pos: usize,        //current moving read pos, local pos
    pub ch: char                //current character

}

impl<'a> LexicalAnalyzer<'a>{
    fn new(cont: &'a str){
        self{
            content: cont.chars()
        }
    }

    fn tokenize(){
        self.read_pos = 0;
        self.pos = 0;
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token(){
            tokens.push(token);
        }
        self.read_pos = 0;

        
    }

    fn next_token(&mut self)->TokenKind{
       
       //self.torture_whitespaces();
        //self.read_pos=0;
        let token = String::new();
        
        read_char();
        let curr_read_char = if let Some(curr_read_char) = read_char(){
            curr_read_char;
        }else{
            return Token{
                name: "".to_string(),
                kind: TokenKind::EndColon
            }
        };

        if curr_read_char.is_alphanumeric{
            //read the alphanumeric stuffs
            let mut text = String::new();
            while let Some(c) = self.get_char(){
                    if c.is_alphanumeric(){
                        self.torture_char();
                        text.push(c);
                    }else{
                        break;
                    }
                }
            return Token{
                name: text
                kind: TokenKind::Indent
            }
            
            }
        }else if curr_read_char.is_numeric{
            //read num
            let mut text = String::new();
            while let Some(c) = self.get_char(){
                if c.is_numeric(){
                    self.torture_char();
                    text.push(c);
                }else{
                    break;
                }
            }
            return Token{
                name: text
                kind: TokenKind::Number
            }
        }
        
        for &(prefix,kind) in FIX_KEYWORD_TOKENS{
            if self.starts_with(prefix){
                self.torture_chars(prefix.len());
                return Token{
                    name: prefix.iter().collect()
                    kind
                }
            }

        }
        /* 
        TODO
        check if the current character is fixed token
         */
        /*while !is_next_char_whitespace(){
            read_char();
            let curr_read_char = if let Some()
        }*/

        //if(token.chars().all(char::is_alphanumeric)){}

        token;
    }

    fn is_next_char_whitespace()->bool{
        if self.content.next()?.is_whitespace(){
            return true;
        }else{
            return false;
        }
    }

    fn torture_whitespaces(){
        while self.get_char().map(|x| x.is_whitespace()).unwrap_or(false){
            self.read_char();
        }
    }

    fn get_char(&self)->Option<char>{
        self.content.nth(pos).cloned();
    }

    fn torture_chars(&self, n){
        for 0..n{
            self.torture_char();
        }
    }

    fn read_char(&self){
        self.ch = get_char();
        self.read_pos+=1;
        self.pos+=1;
    }
}
