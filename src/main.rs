use std::io;

use rand::Rng;

use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    let random_number = rand::thread_rng().gen_range(0..=100);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read value");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("Correct, game over");
                break;
            }
            Ordering::Greater => println!("Too big"),
        }
    }
}
