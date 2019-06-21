#![no_std]

// Imports
extern crate eng_wasm;
extern crate eng_wasm_derive;
extern crate serde;

use eng_wasm::*;
use eng_wasm_derive::pub_interface;
use serde::{Serialize, Desirialize};

// Encrypted state keys
static ELECTION: &str = "election";
static VOTERS &str = "voters";

// Structs
#[derive(Serialize, Desirialize)]
pub struct Election {
   candidates: Vec<Candidate>,
   status: Status,
}

pub enum Status {
    Preparation,
    Progress,
    End,
}

pub struct Candidate {
    symbol: eng_wasm::String,
    obtain: U256,
}

#[derive(Serialize, Desirialize)]
pub struct Voter {
    address: H160,
}

pub struct Contract;

impl Contract {
    fn get_election() -> Election{
        read_state!(ELECTION).unwrap_or_default()
    }
}

#[pub_interface]
pub trait ContractInterface{
    fn add_candidate(symbol: eng_wasm::String)
    fn start_voting()
    fn vote(address: H160, symbol: eng_wasm::String)
    fn compute_winner() -> symbol
}

impl ContractInterface for Contract {
#[no_mangle]
    fn add_candidate(symbol: eng_wasm::String) {
        let mut election = Self::get_election();
        election.candidates.push(Candidate{
            symbol,
            0,
        });
        write_state!(ELECTION => election);
    }
#[no_mangle]
    fn start_voting()  {
        let mut election = Self::get_election();
        election.Status = Status.Progress;
        write_state!(ELECTION => election);
    }
#[no_mangle]
    fn vote(address: H160, symbol: eng_wasm::String) {
        
    }
}
