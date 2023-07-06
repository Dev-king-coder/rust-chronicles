use std::io;

fn main() {
    // println! is a macro
    // macro is a special type of function that is used to generate code
    println!("Guess the number: ");
    println!("Please input your guess: ");

    // mut means mutable
    //String::new() is a function th11at returns a new instance of String
    // :: syntax indicates that new is an associated function of the String type
    let mut guess = String::new();

    // io::stdin() returns an instance of std::io::Stdin
    // read_line() reads input from the user
    // & indicates that this argument is a reference
    // expect() is a method of io::Result
    // expect() is used to catch any errors that may occur
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}