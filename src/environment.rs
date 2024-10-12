use std::{collections::HashMap, process::exit, sync::Mutex};
use lazy_static::lazy_static;


use crate::expr::Literal;

lazy_static! {
    static ref SPACE: Mutex<HashMap<String, Literal>> = Mutex::new(HashMap::new());
}

pub fn define_env(name: String, value: Literal) {
    let mut space = SPACE.lock().unwrap();
    space.insert(name, value);
}

pub fn get_env(name: String) -> Literal {
    let mut space = SPACE.lock().unwrap();
    if let Some(val) = space.get(&name) {
        return val.clone();
    } else {
        eprintln!("Undefined variable {}.", name);
        exit(70);
    }
}
