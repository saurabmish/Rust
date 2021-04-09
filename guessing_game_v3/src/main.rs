use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 11);
    
    loop {
        println!("Guess a number between 1 and 10");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line ...");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Your guess was small"),
            Ordering::Greater => println!("Your guess was big"),
            Ordering::Equal   => {
                println!("YOU WIN !!");
                break;
            }
        }
    }
}

