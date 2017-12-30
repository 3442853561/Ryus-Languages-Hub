#![allow(unused_variables)]
#![allow(dead_code)]

static KEY_WORD: &'static [&'static str] = &[
    "module", "func", "param", "local", "result", "export", "call"
];

static TYPE: &'static [&'static str] = &["i32", "i64", "f32", "f64"];
static FUNC_FOR_TYPE_CLASS: &'static [&'static str] = &["add"];

static FUNC: &'static [&'static str] = &["get_local"];

#[derive(Debug)]
pub enum Expr {
    Key(String),
    Type(String),
    Ident(String),
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
    None
}

fn is_type(code: String) -> Option<Expr> {
    for i in TYPE.iter() {
        if &*code.trim() == *i {
            return Some(Expr::Type(i.to_string()));
        }
    }
    None
}

fn is_ident(code: String) -> Option<Expr> {
    let _code = code.trim();
    if !_code.starts_with("$") {
        return None;
    }
    for i in _code.chars() {
        // match i {
        
        // }
    }
    None
}

fn is_func(code: String) -> Option<Expr> {
    // is global function
    for i in FUNC.iter() {
        if &*code.trim() == *i {
            return Some(Expr::Func(None, i.to_string()));
        }
    }
    // is function for type class
    for class in TYPE.iter() {
        for i in FUNC_FOR_TYPE_CLASS.iter() {
            if code.trim() == class.to_string() + "." + *i {
                return Some(Expr::Func(Some(class.to_string()), i.to_string()));
            }
        }
    }
    None
}

fn get_token(code: String) -> Option<Vec<Expr>> {
    None
}

fn wat2rust(code: String) -> Option<String> {
    None
}

fn main() {
    println!("{:?}", is_key("func".to_string()));
    println!("{:?}", is_type("i32".to_string()));
    println!("{:?}", is_func("get_local".to_string()));
    println!("{:?}", is_func("i32.add".to_string()));
}
