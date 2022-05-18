#![deny(clippy::pedantic)]
#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]
#![feature(is_sorted, box_syntax)]

use std::io::Error;

use algorithms::{huffman_encoding, longest_common_subsequence, quicksort};

pub mod algorithms;
pub mod data_structures;

pub fn run(choice: u32) -> Result<(), Error> {
    match choice {
        2 => quicksort::run(),
        4 => longest_common_subsequence::run(),
        5 => huffman_encoding::run(),
        _ => Ok(()),
    }
}

pub fn display_menu() {
    println!("algorithms:");
    println!("1 - insertion sort");
    println!("2 - quicksort");
    println!("3 - radix sort");
    println!("4 - longest common subsequence");
    println!("5 - huffman encoding");

    println!("data structures:");
    println!("6 - red-black tree");

    println!("0 - that's enough");
}
