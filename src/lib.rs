#![no_std]

// Imports
extern crate eng_wasm;
extern crate eng_wasm_derive;
extern crate serde;

use eng_wasm::*;
use eng_wasm_derive::pub_interface;
use serde::{Serialize, Deserialize};

// Encrypted state keys
static ELECTION: &str = "election";
//static VOTERS: &str = "voters";

// Structs
#[derive(Serialize, Deserialize)]
pub struct Election {
   candidates: Vec<Candidate>,
   status: Status,
}

impl Election {
    fn new() -> Election {
        Election {
            candidates: Vec::new(),
            status:Status::Preparation,
        }
    }

    fn add_candidate(&mut self, symbol: String) -> Result<(), ()>{
        match self.get_candidate(&symbol) {
            Some(_) => {
                Err(())
            },
            None => {
                self.candidates.push(Candidate::new(symbol));
                Ok(())
            },
        }
    }

    fn vote_to_candidate(&mut self, symbol: String){
        match self.get_candidate(&symbol) {
            Some(c) => c.obtain_vote(),
            None => {},
        };
    }

    fn get_candidate(&mut self, symbol: &String) -> Option<&mut Candidate> {
        for i in &mut self.candidates {
            if i.symbol == *symbol {
                return Some(i);
            }
        }
        None
    }

    fn compute_winner(&self) -> String {
        match self.candidates.iter().max_by_key(|m| m.obtain) {
            Some(candidate) => candidate.symbol.clone(),
            None => String::from("")
        }
    }
}
#[test]
fn add_candidate() {
    let mut e = Election::new();
    e.add_candidate(String::from("candidate1"));
    assert!(e.candidates.len() == 1);
    assert!(e.candidates[0].symbol ==
        "candidate1");
    assert!(e.candidates[0].obtain ==
        U256::zero());
}

#[test]
fn vote_to_candidate() {
    let mut e = Election::new();
    e.add_candidate(String::from("candidate1"));
    e.vote_to_candidate(String::from("candidate1"));
    assert!(e.candidates.len() == 1);
    assert!(e.candidates[0].symbol ==
        "candidate1");
    assert!(e.candidates[0].obtain ==
        U256::one());
}

#[test]
fn compute_winner() {
    let mut e = Election::new();
    e.add_candidate(String::from("candidate1"));
    e.add_candidate(String::from("candidate2"));
    e.vote_to_candidate(String::from("candidate1"));
    assert!(e.candidates.len() == 2);
    assert!(e.compute_winner() == "candidate1");
}

#[derive(Serialize, Deserialize)]
pub enum Status {
    Preparation,
    Progress,
    End,
}

#[derive(Serialize, Deserialize)]
pub struct Candidate {
    symbol: String,
    obtain: U256,
}

impl Candidate {
    fn new(symbol: String) -> Candidate {
        Candidate{
            symbol: symbol,
            obtain: U256::zero(),
        }
    }
    fn obtain_vote(&mut self) {
        self.obtain += U256::one();
    }
}

#[test]
    fn obtain_vote_increase_num() {
        let mut c = Candidate {
            symbol: String::from("test"),
            obtain: U256::zero(),
        };
        c.obtain_vote();
        assert!(c.obtain == U256::one())
    }

#[derive(Serialize, Deserialize)]
pub struct Voter {
    address: H160,
}

pub struct Contract;

impl Contract {
    fn get_election() -> Election{
        read_state!(ELECTION).unwrap_or(Election::new())
    }
}

#[pub_interface]
pub trait ContractInterface{
    fn add_candidate(symbol: String);
    fn start_voting();
    fn vote(symbol: String);
    fn compute_winner() -> String;
}

impl ContractInterface for Contract {
#[no_mangle]
    fn add_candidate(symbol: String) {
        let mut election = Self::get_election();
        match election.add_candidate(symbol) {
            Ok(_) => {
                write_state!(ELECTION => election);
            },
            Err(_) => {},
        };
    }
#[no_mangle]
    fn start_voting()  {
        let mut election = Self::get_election();
        election.status = Status::Progress;
        write_state!(ELECTION => election);
    }
#[no_mangle]
    fn vote(symbol: String) {
        let mut election = Self::get_election();
        election.vote_to_candidate(symbol);
        write_state!(ELECTION => election);
    }
#[no_mangle]
    fn compute_winner() -> String {
        Self::get_election().compute_winner()
    }
}
