use std::io; // import standard input-output library
use rand::Rng; // Rng to generate numbers
use std::cmp::Ordering; // standard compare, order >,< ETC

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // using rand; eq to .py: rand(1, 100)

    loop {
    
        println!("Please input your guess.");

        let mut guess = String::new(); // create a mutable variable to store nunmbers

        io::stdin() // read user input
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() { //.expect("Please type a number!"); // only takes integer value
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {guess}");
        // println!("The number was: {secret_number}");

        // Comparing the two numbers
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("Well done, You win!");
                break;
            }
        }
    }
}
