// to get user input and print the result, we use the io library
// comes from the standard library: std
use std::io;
// add the rand library and use it
use rand::Rng;
// add comparing
use std::cmp::Ordering;

fn main() {
    println!("Let's play Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

  // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess!");

        // Next, we’ll create a variable to store the user input, like this
        // variables are immutable by default
        // add keyword mut to make it mutable
        let mut guess = String::new();

        // stdin represents a handle to the input of the terminal. We use the read_line
        // to get what the user inputted and append it to the variable guess
        // the & indicates the arg is a reference. References are immutable by default
        // so we add the mut
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number, let's try again");
                continue
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("That's correct!");
                break;
            }
        }
    }
}
