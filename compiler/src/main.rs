//Compiler main code
use std::env;
mod lexer;

fn main() {
    //Use Command Line arguments to compile source code
    let args: Vec<_> = env::args().collect();

    //Validate if the args are greater than 1. If so, index into it.
    if args.len() > 1 {
        //Begin Lexical Analysis
        lexer::lex(&args[1]);
    
    //Basic Error Checking: There are no args
    } else {
        println!("ERROR: Incorrect file path provided to Turbo Compiler. Check the file name and extension.");
    }

}