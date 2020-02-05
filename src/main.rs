extern crate rand;

use std::cmp::Ordering;
use std::io;
use rand::{Rng, thread_rng};
use guess::Guess;

mod guess;

fn main() {
    println!("Guess with   the number");
    let mut count = 0;
    println!("{}", count);
    let secret_number = thread_rng().gen_range(1, 100);
    loop {
        count = count + 1;
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => continue,
        };

        println!("You guessed: {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You try {} times.You win!", count);
                break;
            }
        }
    }
}
