use std::io; // Import input/output library
use std::cmp::Ordering; // Comparison library
use rand::Rng; // Random generator library

fn main() 
{
    // println! (with !) uses Rust Macro, which is a custom string, not a function
    println!("Guess the number!");

    println!("Input your guess: ");

    // Generate random number
    // 1. thread_rng() -> Calls random number generator function on current thread
    // 2. gen_range() -> picks a number from [x, y)
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop
    {
        // In Rust, variables are immutable by default unless stated otherwise
        // By adding "mut", you are specifiying the variable to be mutable
        let mut guess = String::new();  // Like Java, you use "new" to create instance of a new variable

        // 1. io::stdin() -> Calls io library's stdin function
        // 2. read_line() -> Calls read_line() and only accepts mutable strings as argument
        // 3. &mut guess -> Passes "guess" variable as mutable reference
        // FYI: "mut" is included because "guess" is being passed as a reference. 
        //      Once it reaches to the destinated function, it will be treated as a immutable variable. 
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);
        
        // Override "guess" value and type from string to unsigned 32-bit number
        let guess: u32 = guess
            .trim() // Eliminates whitespace in string
            .parse() { // Parses string into some kind of number, which will be u32 because we specified it
                Ok(num) => num, // If input is valid number, return it
                Err(_) => continue, // If not, move on to next iteration
            };

        // 1. cmp() -> Compares two values of anything. It also return the variable, hence, the curly braces
        // 2. match -> Helps on deciding what to do next with the return value of cmp()
        match guess.cmp(&secret_number)
        {
            // cmp() will return a value and pass it to match
            // match will use the value as reference and compare it to its pattern (left side)
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
