use std::env;

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

//tell lexer which lex file to use
lrlex_mod!("mdl.l");
//tell parser which parser to use
lrpar_mod!("mdl.y");

fn main() {
    let lexerdef = mdl_l::lexerdef();
    let args: Vec<String> = env::args().collect();
    let lexer = lexerdef.lexer(&args[1]);
    let (res, errs) = mdl_y::parse(&lexer);
    for e in errs{
        println!("{}", e.pp(&lexer, &mdl_y::token_epp));
    }
    match res{
        Some(r) => println!("Result: {:?}", r),
        _ => eprintln!("Unable to evaluate expression.")
    }
}
