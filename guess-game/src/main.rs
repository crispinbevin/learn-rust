use rand::Rng;
use std::cmp::{self, Ordering};
use std::io;

fn main() {
    println!("Guessing game! 1-10");

    loop {
        println!("Enter your guess:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Enter an integer");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let secret_number = rand::thread_rng().gen_range(1..=10);

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Try a lower number"),
            Ordering::Less => println!("Try a higher number"),
            Ordering::Equal => {
                println!("You got it right");
                break;
            }
        }
    }
}
