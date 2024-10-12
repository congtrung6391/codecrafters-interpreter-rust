use lazy_static::lazy_static;
use std::{collections::HashMap, process::exit, sync::Mutex};

use crate::expr::Literal;

#[derive(Clone)]
struct State {
    space: HashMap<String, Literal>,
    parent: Option<Box<State>>,
}

impl State {
    pub fn new(par: Option<Box<State>>) -> State {
        State {
            space: HashMap::new(),
            parent: par,
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.space.insert(name, value);
    }

    pub fn get(&mut self, name: String) -> Literal {
        if let Some(val) = self.space.get(&name) {
            return val.clone();
        }
        match self.parent.clone() {
            None => {
            eprintln!("Undefined variable {}.", name);
            exit(70);
            }
            Some(mut par) => {
                return par.get(name);
            }
        }
    }

    pub fn get_parent(&self) -> State {
        match self.parent.clone() {
            None => {
                exit(70);
            }
            Some(par) => {
                let p = *par;
                return p;
            }
        }
    }
}

lazy_static! {
    static ref SPACE: Mutex<HashMap<String, Literal>> = Mutex::new(HashMap::new());
}

lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(State::new(None));
}

pub fn define_env(name: String, value: Literal) {
    let mut space = STATE.lock().unwrap();
    space.define(name, value);
}

pub fn get_env(name: String) -> Literal {
    let mut space = STATE.lock().unwrap();
    return space.get(name);
}

pub fn add_block() {
    let mut state = STATE.lock().unwrap().clone();
    let new_block = State::new(Some(Box::new(state)));
    state = new_block;
}

pub fn remove_block() {
    let mut state = STATE.lock().unwrap().clone();
    let prev_state = state.get_parent();
    state = prev_state;
}
