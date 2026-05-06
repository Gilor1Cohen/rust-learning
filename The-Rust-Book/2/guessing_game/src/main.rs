use std::io;
/// std = standard library.
/// io = input/output library, allows us to get user input.

use std::cmp::Ordering;
/// cmp = compare function that compares two values and returns an Ordering.
/// Ordering = enum that has three variants: Less, Greater, and Equal. These variants are

use rand::Rng;
/// rand = random number generator library.
/// Rng = trait that defines methods that random number generators implement.

fn main() {
    println!("Guess the number!");

    let secret_number: u8  = rand::thread_rng().gen_range(1..=100);
    // creates a random number generator for the current thread and uses it to generate a random number from 1 to 100, including both 1 and 100.

    let mut guess: String = String::new();
    // let mut guess is mutable variable named guess that can be changed later in the program.
    // String::new is a function that returns a new instance of a String. This new function creates a new, empty string. 

    loop { 
        guess.clear();
        // guess.clear() is used to clear the contents of the guess variable before each new input. This ensures that previous guesses do not interfere with the current guess.


        println!("Please input your guess.");
        
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        //  &mut guess is a mutable reference to the guess variable. This allows the read_line function to modify the guess variable directly.
        // & is used to create a reference to the guess variable, and mut allows that reference to be mutable, meaning it can be changed.


 
        match guess
            .trim()
            .parse::<u8>()
            .expect("Please enter a valid number!")
            .cmp(&secret_number)
        {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }   
            // guess.cmp(&secret_number) compares guess to the secret number secret_number.
            // Ordering::Less means guess is smaller than secret_number.
            // Ordering::Greater means guess is bigger than secret_number.
            // Ordering::Equal means guess is exactly equal to secret_number.
        }
}