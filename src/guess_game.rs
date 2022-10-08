use std::cmp::Ordering;
use std::io;

use rand::Rng;

pub fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("This is guessing game. Guess the number {secret_number}");

    loop {
        println!("Your guess:");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("You've guessed it!");
                break;
            }
        }
    }
}
