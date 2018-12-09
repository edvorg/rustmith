#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod common;

use crate::common::Note;

fn main() {
    let freqs = Note::make_test_frequencies();
    let s = serde_json::to_string_pretty(&freqs).unwrap();
    println!("frequences\n{}", s);
}
