use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut name = String::new();
    let imname = "Luiz Felipe"; // this variable is imutable!!

    println!("I am {imname}, what is your name?");
    io::stdin()
        .read_line(&mut name)
        // puts whatever the user passes into a string
        // but it also returns a Result value
        // returns Ok or Err
        // if Err, then the expect (method of Result) will log the message below vvv
        .expect("Failed to read the line"); //without passing this, will compile with warnings

    let name = name.trim();

    // variables and references are immutable by default!

    loop {
        println!("{name}, input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            // here we are creating a mutable reference
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // shadowing a variable!
        // also this trims and parses the string to a number!
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }
    }
}
