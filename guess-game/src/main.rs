use rand::{Rng, thread_rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing Game!\n Enter a number:");
    let secret_number = thread_rng().gen_range(1..=10);

    loop {
        let mut guess = String::new();
        println!("Guess a number from 1 to 10");
        io::stdin()
            .read_line(&mut guess)
            .expect("Give an input dumbass");

        let guess: u32 = guess.trim().parse().expect("Use an integer");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Try a higher nunber"),
            Ordering::Greater => println!("Try a lower number"),
            Ordering::Equal => {
                println!("You got it right!");
                break;
            }
        }
    }
}
