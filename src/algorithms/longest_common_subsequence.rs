use colour::green;
use std::{
    cmp,
    io::{self, Error},
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Arrow {
    Vertical,
    Horizontal,
    Diagonal,
    Empty,
}

#[derive(Clone, Copy)]
struct Cell {
    length: u32,
    direction: Arrow,
    path: bool,
}

impl Cell {
    fn new() -> Self {
        Self {
            length: 0,
            direction: Arrow::Empty,
            path: false,
        }
    }
}

#[must_use]
pub fn lcs(a: &str, b: &str) -> String {
    let mut row_chars: Vec<char> = a.chars().collect();
    let column_chars: Vec<char> = b.chars().collect();

    let mut table: Vec<Vec<Cell>> =
        vec![vec![Cell::new(); column_chars.len() + 1]; row_chars.len() + 1];

    for (i1, c1) in row_chars.iter().enumerate() {
        for (i2, c2) in column_chars.iter().enumerate() {
            if c1 == c2 {
                table[i1 + 1][i2 + 1].length = table[i1][i2].length + 1;
                table[i1 + 1][i2 + 1].direction = Arrow::Diagonal;
            } else {
                table[i1 + 1][i2 + 1].length =
                    cmp::max(table[i1 + 1][i2].length, table[i1][i2 + 1].length);
                table[i1 + 1][i2 + 1].direction =
                    if table[i1 + 1][i2].length > table[i1][i2 + 1].length {
                        Arrow::Horizontal
                    } else {
                        Arrow::Vertical
                    }
            }
        }
    }

    let mut lcs: Vec<char> = Vec::new();
    let (mut x, mut y) = (a.len(), b.len());

    while x > 0 && y > 0 {
        table[x][y].path = true;
        if row_chars[x - 1] == column_chars[y - 1] {
            lcs.push(row_chars[x - 1]);
        }
        match table[x][y].direction {
            Arrow::Diagonal => {
                x -= 1;
                y -= 1;
            }
            Arrow::Vertical => x -= 1,
            Arrow::Horizontal => y -= 1,
            Arrow::Empty => (),
        }
    }

    print!("        ");
    for l in &column_chars {
        print!("{}   ", l);
    }
    println!();

    row_chars.insert(0, ' ');
    for (i, v) in table.iter().enumerate() {
        print!("{} ", row_chars[i]);
        for n in v {
            match n.direction {
                Arrow::Diagonal => print!("\\"),
                Arrow::Horizontal => print!("-"),
                Arrow::Vertical => print!("|"),
                Arrow::Empty => print!(" "),
            }
            if n.path {
                green!("{:^3}", n.length);
            } else {
                print!("{:^3}", n.length);
            }
        }
        println!();
    }

    lcs.iter().rev().collect::<String>()
}

pub fn run() -> Result<(), Error> {
    let mut string1 = String::new();
    let mut string2 = String::new();
    println!("string 1:");
    io::stdin()
        .read_line(&mut string1)
        .expect("something went wrong");

    println!("string 2:");
    io::stdin()
        .read_line(&mut string2)
        .expect("something went wrong");

    string1 = string1.trim().to_string();
    string2 = string2.trim().to_string();

    let lcs = lcs(&string1, &string2);

    if lcs.is_empty() {
        println!("no common subsequences");
    } else {
        println!("longest common subsequence: {:?}", lcs);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(lcs("gac", "agcat"), "ga");
    }

    #[test]
    fn nawigator() {
        assert_eq!(lcs("nawigator", "nowator"), "nwator");
    }
}
