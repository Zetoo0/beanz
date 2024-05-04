use std::borrow::Borrow;

use lexerNex::LexAnalyzer;

mod lexerNex;

fn main() {
    let ass = "pool)(";
    let mut anal: LexAnalyzer = LexAnalyzer::new(ass);
    let analayzed = anal.tokenize();
    println!("{:?}",analayzed[analayzed.len()-1]);
    for i in 0..analayzed.len(){
        println!("{:?}", analayzed[i]);
    }
}
