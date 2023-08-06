#![allow(non_camel_case_types)]
#[derive(Debug)]

// Library containing possible tokens for analysis
pub enum Token {
    // Special symbols:
    tok_ss_parenthesis,
    tok_ss_braces,
    tok_ss_comma,
    tok_ss_semicolon,

    // Operators:
    tok_op_arithmetic,
    tok_op_relational,
    tok_op_logical,
    tok_op_assign,

    // Management:
    tok_man_eof,
    tok_man_identifier,
    tok_man_number,

    // Keywords:
    tok_res_def,
    tok_res_return,
    tok_res_if,
    tok_res_else,
    tok_res_let,
    tok_res_and,
    tok_res_or
}