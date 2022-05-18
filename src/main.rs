#![deny(clippy::pedantic)]
#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]
use std::io::{self, Error};
use informatics::{run, display_menu};

fn main() -> Result<(), Error>{
    loop {
        display_menu();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("wrong choice");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("numbers only"),
        };
        if choice == 0 {
            break
        }
        run(choice)?;
    }
    Ok(())
}