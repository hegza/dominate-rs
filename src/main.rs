#![feature(plugin)]
#![plugin(phf_macros)]

extern crate phf;

mod cli;

use std::fs::File;
use std::io::prelude::*;
use cli::CLI;

fn main() {

    // Load an example file
    let mut file = File::open("data/test.in").expect("Unable to open file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file.");

    // Run the command line interface
    let mut cli = CLI::new(&contents);
    cli.run();

}
