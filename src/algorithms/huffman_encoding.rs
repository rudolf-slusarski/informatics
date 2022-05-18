#![deny(clippy::pedantic)]
#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]

use std::{
    cmp,
    collections::{BTreeMap, BinaryHeap},
    fs::{self, File},
    io::{self, Error},
    time::Instant,
};

use bit_vec::BitVec;
use bitstream_io::{BigEndian, BitWrite, BitWriter};

#[derive(Debug, PartialEq, Eq)]
struct Node {
    frequency: usize,
    kind: NodeKind,
}

#[derive(Debug, PartialEq, Eq)]
enum NodeKind {
    Internal(Box<Node>, Box<Node>),
    Leaf(char),
}

impl Ord for Node {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        rhs.frequency.cmp(&self.frequency)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

#[must_use]
fn huffman_encoding(data: &str) -> (BTreeMap<char, BitVec>, BTreeMap<char, usize>) {
    // counting
    let mut frequencies: BTreeMap<char, usize> = BTreeMap::new();
    for c in data.chars() {
        *frequencies.entry(c).or_insert(0) += 1;
    }

    // queue
    let mut priority_queue = BinaryHeap::new();
    for c in &frequencies {
        priority_queue.push(Node {
            frequency: *c.1,
            kind: NodeKind::Leaf(*c.0),
        });
    }

    // huffman
    while priority_queue.len() > 1 {
        let left = priority_queue.pop().unwrap();
        let right = priority_queue.pop().unwrap();
        priority_queue.push(Node {
            frequency: left.frequency + right.frequency,
            kind: NodeKind::Internal(box left, box right),
        });
    }
    let mut codes: BTreeMap<char, BitVec> = BTreeMap::new();
    generate_code(priority_queue.peek().unwrap(), BitVec::new(), &mut codes);

    (codes, frequencies)
}

fn generate_code(node: &Node, prefix: BitVec, out_codes: &mut BTreeMap<char, BitVec>) {
    match node.kind {
        NodeKind::Internal(ref left, ref right) => {
            let mut left_prefix = prefix.clone();
            left_prefix.push(false);
            generate_code(left, left_prefix, out_codes);

            let mut right_prefix = prefix;
            right_prefix.push(true);
            generate_code(right, right_prefix, out_codes);
        }
        NodeKind::Leaf(c) => {
            out_codes.insert(c, prefix);
        }
    }
}

fn compare_file_sizes(file1: &File, file2: &File) -> Result<f64, Error> {
    let file1_size = file1.metadata()?.len();
    let file2_size = file2.metadata()?.len();
    let compression_percentage = (cmp::max(file1_size, file2_size)
        - cmp::min(file1_size, file2_size)) as f64
        / cmp::max(file1_size, file2_size) as f64
        * 100.0;

    Ok(compression_percentage)
}

fn print_stats(frequencies: &BTreeMap<char, usize>, codes: &BTreeMap<char, BitVec>) {
    println!(" frequency       code");
    for i in codes {
        println!(
            "{:>9}x _ {} _ {:?}",
            frequencies.get(i.0).unwrap(),
            i.0,
            i.1
        );
    }
}

pub fn run() -> Result<(), Error> {
    let mut source_file = String::new();
    println!("source file:");
    io::stdin()
        .read_line(&mut source_file)
        .expect("something went wrong");
    source_file.pop();
    source_file = format!("../../{}", source_file);

    let total_time = Instant::now();
    let text = fs::read_to_string(&source_file)?;

    let encoding_time = Instant::now();
    let (codes, frequencies): (BTreeMap<char, BitVec>, BTreeMap<char, usize>) =
        huffman_encoding(&text);
    println!("encoding time: {:?}", encoding_time.elapsed());

    let file = File::create("../../encoded_file")?;
    let mut writer = BitWriter::endian(file, BigEndian);
    for c in text.chars() {
        for bit in codes.get(&c).unwrap().iter() {
            writer.write_bit(bit)?;
        }
    }

    print_stats(&frequencies, &codes);

    println!(
        "compressed by {:?}%",
        compare_file_sizes(
            &File::open(source_file).unwrap(),
            &File::open("../../encoded_file").unwrap()
        )
        .unwrap()
    );
    println!("total time: {:?}", total_time.elapsed());
    Ok(())
}
