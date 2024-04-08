use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome To Guess The Number Game!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please Enter the guess number:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 =match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You Guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        };
        println!("The Secret Number {}", secret_number);
    }
}
