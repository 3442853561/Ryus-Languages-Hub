mod token;

use token::*;

fn get_token(code: String) -> Option<Vec<Expr>> {
    let my_token = "".to_string();
    for i in code.chars() {
	    match i {
		    ' ' | '\n' | '\t' => {}
			'(' => {}
			')' => {}
			_ => {}
		}
	}
    None
}

fn wat2rust(code: String) -> Option<String> {
    None
}

fn main() {
    
}
