use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");

    loop {
        let mut guess = String::new(); // this will create a new instance of string variable

        io::stdin()
            .read_line(&mut guess) // returns a Result value which is an enum
            .expect("Failed to read line"); // you use .expect to define an error message if it results an error

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // _ means catchall value which means it will catch all Err values
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                println!("Try Again.");
            }
            Ordering::Greater => {
                println!("Too big!");
                println!("Try Again.");
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}
