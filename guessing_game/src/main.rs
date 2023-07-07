use std::io;
// Rng trait defines a method that generates a Random Number Generator
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // println! is a macro
    // macro is a special type of function that is used to generate code
    println!("....GUESS THE NUMBER GAME....");

    // thread_rng: rand functiion local to current thread of execution and is seeded in OS
    // gen_range: rng method for geenrating a random number between passed argument
    // cargo doc --open generates documentation of all crates included in project
    // start..=end (inclusive for both upper and lower bounds)
    let secret_number = rand::thread_rng().gen_range(1..=10);
    println!("RANDOM NUMBER GENERATED!");

    // mut means mutable
    //String::new() is a function th11at returns a new instance of String
    // :: syntax indicates that new is an associated function of the String type
    // io::stdin() returns an instance of std::io::Stdin
    // read_line() reads input from the user
    // & indicates that this argument is a reference
    // expect() is a method of io::Result
    // expect() is used to catch any errors that may occur
    loop {
        println!("Please input your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // shadowing the value of alredy existing variable guess
        // "guess.trim().parse()" parses the trimmed string "guess"
        // u32=unsigned 32 bit data type
        // let guess: u32 = guess.trim().parse().expect("Please type a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Guessed Number is...CHOTU..Just like your D**k size L)"),
            Ordering::Greater => println!("Guessed Number is...BIGGY..Just like my D**k size :)"),
            Ordering::Equal => {
                println!("Guessed Number is...Perfect Baby aahhh");
                break;
            }
        };
    }
    println!("The secret number is :{secret_number}");
}
