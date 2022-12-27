// WHITELIST SPECIAL RULES
//#![allow(unused)]

// IMPORTS
use std::io;

// EXAMPLE IMPORTS
// use std::io::{Write, BufReader, BufRead, ErrorKind};
// use std::fs::File;
// use std::cmp::Ordering;

fn main() {
    println!("WHAT IS YOUR NAME?");

    // RESERVE MEMORY FOR USERNAME
    let mut name = String::new();

    // const foo: u32 = 1000;

    // READ USER INPUT
    io::stdin().read_line(&mut name)
        .expect("DID NOT RECEIVE INPUT");

    // PROMPT MESSAGE
    println!("HELLO {}", name);
}