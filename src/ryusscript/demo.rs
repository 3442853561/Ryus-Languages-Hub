static KEY_WORD: &'static [&'static str] = &[
    "allow", 
	"as", 
	"bool", 
	"break", 
	"class", 
	"else", 
	"fn", 
	"if", 
	"implements", 
	"interface", 
	"loop",
	"null", 
	"public", 
	"return", 
	"static", 
	"use" 
];

static UNSAFE_KEY_WORD: &'static [(&'static str, &'static str)] = &[
    ("//extends", "enum"), 
    ("//extends", "struct"), 
];

static UNSTABLE_KEY_WORD: &'static [(&'static str, &'static str)] = &[
	("//box", "box"), 
    ("//extends", "extends"), 
];

#[derive(Debug)]
pub enum Expr {
    Key(String),
    Type(String),
    Ident(String, bool),
    Func(Option<String>, String),
    Integer(i64),
    Float(f64),
    Child(Vec<Expr>),
}

fn is_key(code: String) -> Option<Expr> {
    for i in KEY_WORD.iter() {
        if &*code.trim() == *i {
            return Some(Expr::Key(i.to_string()));
        }
    }
	for &(_, i) in UNSAFE_KEY_WORD.iter() {
        if &*code.trim() == i {
            return Some(Expr::Key(i.to_string()));
        }
    }
	for &(_, i) in UNSTABLE_KEY_WORD.iter() {
        if &*code.trim() == i {
            return Some(Expr::Key(i.to_string()));
        }
    }
	None
}


fn main() {
	println!("{:?}", is_key("enum".to_string()));
}
