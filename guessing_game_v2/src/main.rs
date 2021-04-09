use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 11);

    println!("Guess a number between 1 and 10");
    
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number");

    println!("You guessed {}, and the secret number was {}", guess, secret_number);

    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Your guess was too small!"),
        Ordering::Equal   => println!("YOU WIN !!"),
        Ordering::Greater => println!("Your guess was too big!!!"),
    }
}
