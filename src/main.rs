use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let expect_read_line_msg = String::from("Failed to read line");
    let range = 1..=10;
    let secret_number = rand::thread_rng().gen_range(range);
    let io_instance = io::stdin();

    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io_instance
            .read_line(&mut guess)
            .expect(&expect_read_line_msg);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("error input, please type again!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("toll big!"),
            Ordering::Equal => {
                println!("Equal!");
                break;
            }
        }
    }

    println!("the secret number: {}", secret_number);
}
