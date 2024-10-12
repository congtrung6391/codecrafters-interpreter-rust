use lazy_static::lazy_static;
use std::{collections::HashMap, process::exit, sync::{Arc, Mutex}};

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

    pub fn assign(&mut self, name: String, value: Literal) {
        if let Some(val) = self.space.get(&name) {
            self.space.insert(name.clone(), value);
            return;
        }
        if let Some(ref mut par) = self.parent {
            par.assign(name.clone(), value);
            return;
        }
        eprintln!("Undefined variable {}.", name.clone());
        exit(70);
    }

    pub fn get_parent(&self) -> State {
        match &self.parent {
            None => {
                exit(70);
            }
            Some(par) => {
                let p = *par.clone();
                return p;
            }
        }
    }
}

lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(State::new(None));
}

pub fn define_env(name: String, value: Literal) {
    let mut space = STATE.lock().unwrap();
    space.define(name, value);
}

pub fn assign_env(name: String, value: Literal) {
    let mut space = STATE.lock().unwrap();
    space.assign(name, value);
}

pub fn get_env(name: String) -> Literal {
    let mut space = STATE.lock().unwrap();
    return space.get(name);
}

pub fn add_block_scoping() {
    let mut state = STATE.lock().unwrap().clone();
    let new_block = State::new(Some(Box::new(state)));
    *STATE.lock().unwrap() = new_block.clone();
}

pub fn remove_block_scoping() {
    let mut state = STATE.lock().unwrap().clone();
    let prev_state = state.get_parent();
    *STATE.lock().unwrap() = prev_state;
}
