// brings io and Rng traits into scope
// brings the ordering enum type into scope
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // thread_rng(): a random nr generator local to the current thread and seeded from the OS
    // gen_range(): method defined by the  Rng trait
    // cargo doc --open in this dir compiles docs from all the dependencies and opens in browser
    // Rust infers that the type of the variable is a number, defaults to i32 type
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // loop: Starts an infinite loop
    loop {
        println!("Please input your guess.");

        // let: variable definition
        // mut: makes it mutable, by default all is immutable
        // String: a type
        // ::new: an "associated function" of the String type. Equivalent of static function.
        // Rust infers that the type of guess is string.
        let mut guess = String::new();

        // Could be std::io::stdin if std::io was not brought into scope
        io::stdin()
            // &: argument is a reference, immutable by default
            // &mut: makes reference mutable
            // returns io::Result type, an enum with variants Ok or Err.
            .read_line(&mut guess)
            // calls the expect method of the Result type.
            // On Err value, causes a crash and displays the msg
            // On Ok value, returns the value that Ok is holding (nr of read bytes)
            .expect("Failed to read line");

        // shadow the previous guess variable
        // trim: remove whitespace
        // parse: parses a string into one of the number types.
        //        Returns a Result type.
        // : u32: annotate the variable's type to u32.
        //        Because of the comparison later,
        //          Rust infers that the type of the compared number is also u32.
        // match: expression to handle variants of the Result enum returned by parse()
        let guess: u32 = match guess.trim().parse() {
            // Ok: pattern to handle the Ok type returned by parse, which contains the parsed number
            Ok(num) => num,
            // Err(_) : pattern, catching all types of Err values
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // cmp: returns an Ordering type
        // Ordering: an enum with 3 variants: Less, Greater, Equal
        // match: expression, made up of arms.
        //      Arm = pattern + code that should run if the value matches the pattern
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // break the loop
                break;
            }
        }
    }
}
