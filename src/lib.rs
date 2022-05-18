#![deny(clippy::pedantic)]
#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]
#![feature(is_sorted, box_syntax)]

use std::io::Error;

use algorithms::{huffman_encoding, longest_common_subsequence, quicksort};

pub mod algorithms;
pub mod data_structures;

pub fn run(choice: u32) -> Result<(), Error> {
    match choice {
        3 => quicksort::run(),
        5 => longest_common_subsequence::run(),
        6 => huffman_encoding::run(),
        _ => Ok(()),
    }
}

pub fn display_menu() {
    println!("algorithms:");
    println!("1 - insertion sort");
    println!("2 - heapsort");
    println!("3 - quicksort");
    println!("4 - radix sort");
    println!("5 - longest common subsequence");
    println!("6 - huffman encoding");

    println!("data structures:");
    println!("7 - red-black tree");

    println!("0 - that's enough");
}
