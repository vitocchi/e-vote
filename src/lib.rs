#![no_std]

// Imports
extern crate eng_wasm;
extern crate eng_wasm_derive;
extern crate serde;

use eng_wasm::*;
use eng_wasm_derive::pub_interface;
use serde::{Serialize, Desirialize};

// Encrypted state keys
static VOTES: &str = "votes";

// Structs
#[derive(Serialize, Desirialize)]
pub struct Vote Vec<Candidate>

pub struct Candidate {

}