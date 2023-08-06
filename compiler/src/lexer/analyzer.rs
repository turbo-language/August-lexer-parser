use crate::lexer::token_lib::Token;

//Helper function to validate if another iteration can be completed without going out of bounds
fn can_iterate(size: usize, pos: usize) -> bool {
    if pos + 1 >= size {
        return false;
    }
    return true;
}

//Analyzing the characters scanned from the source code for specific tokens.
pub fn analysis(buffer: Vec<char>) -> Vec<(String, Token)> {
    
    //Recursive Variable initialization:
    let mut tokens: Vec<(String, Token)> = Vec::new();
    let mut curr_pos = 0;

    //Analysis Variable initialization:
    let mut token_string: String = String::new();

    //If the buffer contains any characters (ie. if the source code was not empty:)
    if buffer.len() > 0 {
        //A buffer character to take in the inputted characters
        let mut buff_char: char = ' ';
        
        //Iterate to the end of the file checking for the various tokens
        while curr_pos < buffer.len() {
            buff_char = buffer[curr_pos];
            if can_iterate(buffer.len(), curr_pos) {
                curr_pos += 1;
            } else {
                break;
            }

            //Check for Comments: If the current character is '/' and the one after it is also '/', then comment till the end of the line ('\n')
            if buff_char == '/' && buffer[curr_pos] == '/' {

                //Different buff_char to specifically handle ignoring comments 
                let mut comment_buff_char = buff_char;
                while comment_buff_char != '\n' && curr_pos + 1 < buffer.len() {
                    comment_buff_char = buffer[curr_pos];
                    if can_iterate(buffer.len(), curr_pos) {
                        curr_pos += 1;
                    } else {
                        break;
                    }
                }
            }

            //Check for keywords and alphanumeric identifiers:
            //First check if the initial character is alphabetic
            if buff_char.is_alphabetic() {

                let mut is_identifier = true;

                //Begin constucting the token string for analysis
                token_string.push(buff_char);
                buff_char = buffer[curr_pos];

                //keep feeding in characters to the token string, and checking for if its a valid token
                while buff_char.is_alphanumeric() {
                    token_string.push(buff_char);
                    
                    //Actual Analysis: Match the token string with various possible token types
                    match token_string.as_str() {
                        //Example Token: def
                        "def" => {
                            //Firstly, once  a token is detected, the analyzer pushes the corresponding token type into the tokens vector
                            is_identifier = false;
                            tokens.push(("def".to_string(), Token::tok_res_def));
                            //Next, the token string is cleared for further analysis
                            token_string.clear();
                            
                            //To set up the next iteration, a check is performed to ensure that iteration is still possible and then updates the next buffer character
                            if can_iterate(buffer.len(), curr_pos) {
                                //If so, the position is
                                curr_pos += 1;
                            } else {
                                break;
                            }
                            buff_char = buffer[curr_pos];
                            break;
                        }
                        "return" => {
                            is_identifier = false;
                            tokens.push(("return".to_string(), Token::tok_res_return));
                            token_string.clear();

                            if can_iterate(buffer.len(), curr_pos) {
                                curr_pos += 1;
                            } else {
                                break;
                            }
                            buff_char = buffer[curr_pos];
                            break;
                        }
                        "let" => {
                            is_identifier = false;
                            tokens.push(("let".to_string(), Token::tok_res_let));
                            token_string.clear();

                            if can_iterate(buffer.len(), curr_pos) {
                                curr_pos += 1;
                            } else {
                                break;
                            }
                            buff_char = buffer[curr_pos];
                            break;
                        }
                        "if" => {
                            is_identifier = false;
                            tokens.push(("if".to_string(), Token::tok_res_if));
                            token_string.clear();

                            if can_iterate(buffer.len(), curr_pos) {
                                curr_pos += 1;
                            } else {
                                break;
                            }
                            buff_char = buffer[curr_pos];
                            break;
                        }
                        "else" => {
                            is_identifier = false;
                            tokens.push(("else".to_string(), Token::tok_res_else));
                            token_string.clear();

                            if can_iterate(buffer.len(), curr_pos) {
                                curr_pos += 1;
                            } else {
                                break;
                            }
                            buff_char = buffer[curr_pos];
                            break;
                        }
                        "and" => {
                            is_identifier = false;
                            tokens.push(("and".to_string(), Token::tok_res_and));
                            token_string.clear();

                            if can_iterate(buffer.len(), curr_pos) {
                                curr_pos += 1;
                            } else {
                                break;
                            }
                            buff_char = buffer[curr_pos];
                            break;
                        }
                        "or" => {
                            is_identifier = false;
                            tokens.push(("or".to_string(), Token::tok_res_or));
                            token_string.clear();

                            if can_iterate(buffer.len(), curr_pos) {
                                curr_pos += 1;
                            } else {
                                break;
                            }
                            buff_char = buffer[curr_pos];
                            break;
                        }
                        //... All other token types:

                        //If not currently matching, keep iterating and checking
                        _ => {

                            if can_iterate(buffer.len(), curr_pos) {
                                curr_pos += 1;
                            } else {
                                break;
                            }
                            buff_char = buffer[curr_pos];
                        }
                    }
                }

                //If it isn't anything else, and the current character is not alphanumeric, it must be an identifier
                if is_identifier {
                    tokens.push((token_string.clone(), Token::tok_man_identifier));
                    token_string.clear();
                }
            }
        }
    } 

    tokens.push(("EOF Character".to_string(), Token::tok_man_eof));
    return tokens;
}