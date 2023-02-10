use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("Guess the number game");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess a number");

        let mut guess = String::new();

        stdin()
            .read_line(&mut guess)
            .expect("Error in reading user input");

        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("too big"),
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
