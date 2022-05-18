use informatics::{display_menu, run};
use std::io::{self, Error};

fn main() -> Result<(), Error> {
    loop {
        display_menu();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("wrong choice");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("numbers only"),
        };
        if choice == 0 {
            break;
        }
        run(choice)?;
    }
    Ok(())
}
