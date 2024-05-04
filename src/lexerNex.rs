use std::{ops::Add, str::Chars};

#[derive(Debug)]
pub enum TokenKind{
    Indent,
    Number,
    Operator,
    OpenSeparator,
    CloseSeparator,
    Keyword,
    EndColon,
    Error,
    Unidentified
}

impl TokenKind{

    fn analyze(&self)->&str{
        match self{
            Self::Indent => "indent",
            Self::Number => "number",
            Self::Operator => "operator",
            Self::OpenSeparator => "open separator",
            Self::CloseSeparator => "close separator",
            Self::Keyword => "keyword",
            Self::EndColon => "ender",
            _ => "Hé ecsém füttyent a kompájler",
        }
    }
}



#[derive(Debug)]
pub struct Token{
    token: TokenKind,
    name: String
}


pub struct LexAnalyzer<'a>{
    input: Chars<'a>,
    curr_char: usize,
    read_char: usize,
    ch: char
}

impl<'a> LexAnalyzer<'a>{

    pub fn new(cont:&'a str)->LexAnalyzer{
        LexAnalyzer{
            input: cont.chars(),
            curr_char: 0,
            read_char: 0,
            ch: ' '
        }
    }

    pub fn tokenize(&mut self)->Vec<Token>{
    
        let mut tokens = Vec::new();
       /* let end_check: char = self.input.next_back().unwrap();
        if end_check != ';'{
            tokens.push(Token{name:"e".to_string(), token: TokenKind::Error});
        }*/
      //  println!("{:?}", self.input.scan(initial_state, f));
        while let Some(token) = self.next_token(){
            println!("{:?}", token);
            tokens.push(token);
        }
        println!("vissza kellene adnom kekw");
        tokens

    }
    pub fn next_token(&mut self)->Option<Token>{
        //self.input;
        self.ch = self.input.next()?;
        println!("first char: {:?}", self.ch);
        //self.ch = self.input.next()?;
        if self.ch == ';'{
            return Some(Token{
                name: "".to_string(),
                token: TokenKind::EndColon
            })
        }else if self.ch.is_whitespace(){
            println!("szpész");
          //  self.ch = self.input.next()?;
        }
        
            
        if self.ch.is_ascii_alphabetic(){
            let mut text = String::new();
            println!("first as alphanum: {:?}", self.ch);
            
            /*loop{
                self.ch = self.input.next()?;
                if(self.ch.is_whitespace()){
                    self.ch = self.input.next_back()?;
                    break;
                }else{
                    text.push(self.ch);
                }
            }*/
             while self.ch.is_ascii_alphabetic(){
                println!("curr char: {:?}", self.ch);
                text.push(self.ch);
                    //self.curr_char += 1;
                self.ch = self.input.next()?;
            }
            match text.as_str(){
                 "uccuneki" =>{
                    return Some(Token{
                        name: text,
                        token: TokenKind::Keyword
                    })
                },
                "netan" =>{
                    return Some(Token { token: TokenKind::Keyword, name: text })
                }
                _ =>{
                    println!("Visszaadom: {:?}", text);
                    return Some(Token{
                        name: text,
                        token: TokenKind::Indent
                    })
                }
            }

        }
        else if self.ch.is_numeric(){
            println!("???");
            let mut text = String::new();
            println!("aaaa {:?}", text);
            while self.ch.is_numeric(){
                text.push(self.ch);
              //  self.curr_char += 1;
                self.ch = self.input.next()?;
            }
            println!("aaaa {:?}", text);
            return Some(Token{
                name: text,
                token: TokenKind::Number
            })
        }
        else{
           // self.curr_char += 1;
            println!("operator problem");
            match self.ch{
                '(' => {
                    return Some(Token{
                        name: "open separator".to_string(),
                        token: TokenKind::OpenSeparator
                    })   
                },
                ')' =>{
                    return Some(Token{
                        name: "close separator".to_string(),
                        token: TokenKind::CloseSeparator
                    })
                },
                '+' =>{
                    return Some(Token{
                        name: "operator".to_string(),
                        token: TokenKind::Operator
                    })
                },
                '-' =>{
                    return Some(Token{
                        name: "operator".to_string(),
                        token: TokenKind::Operator
                    })
                },
                '*' =>{
                    return Some(Token{
                        name: "operator".to_string(),
                        token: TokenKind::Operator
                    })
                },
                '=' =>{
                    println!("?");
                    return Some(Token{
                        name: "operator".to_string(),
                        token: TokenKind::Operator
                    })
                },
                 _ =>{
                    return Some(Token { token: TokenKind::Unidentified, name: self.ch.to_string() })
                }   
            }
            
           
        }
    }
}

   /*  pub fn get_char(&self)->Option<char>{
        return self.input.next();
    }*/

    /*pub fn read_char(&self){
        self.ch = self.input.next()?;
        self.curr_char+=1;
        self.read_char+=1;
        
    }*/








