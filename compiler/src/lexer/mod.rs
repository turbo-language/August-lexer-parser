mod scanner;
mod analyzer;
mod token_lib;

//Lexical Analysis:
pub fn lex(file_path: &str) {
    //Buffer of Characters (Inputting the source code)
    let buffer = scanner::scan(file_path);

    //Lexical Analysis to produce the tokens
    let tokens: Vec<(String, token_lib::Token)> = analyzer::analysis(buffer);

    println!("{:?}", tokens);
    //Parsing the tokens to create a Parse Tree/Abstract Syntax Tree
}