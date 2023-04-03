// C like language compiled to hvm
//
// let x = 1 + 2;
// print(x);

mod lexer;
use lexer::*;


fn main() {
    let input = b"let x = 2;#";
    let lexer = Lexer::lex_tokens(input);
}
