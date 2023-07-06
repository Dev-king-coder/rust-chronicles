//Preluded set of items come with standard library.
use std::io;
//Using rand crate
use rand::Rng;
use std::cmp::Ordering;
fn main() {

    //println! is a macro that prints to screen.
    println!("Guess The Number!");

    //let to declare var not wars.
    // mut to make vars mutable,default immutable.
    //String::new() is a function that binds a new instance to a string.
    //"::" represents associtaed function,implements on a type.
    //new function creates an empty instance.
    //function stdin() for handling user input
    loop{
        let mut guess=String::new();
        println!("Give a guess b/w 1-10- ");

        io::stdin()
            //.read_line is a method
            //full job of read_line is to take whatever the user types into standard input 
            //and append that into a string (without overwriting its contents),
            //& before mut argument is referenced,without needing to copy the data int memory multiple times
            // references are also mutable to &mut guess refers guess.
            .read_line(&mut  guess)
            //read_line method returns an enum "Result"-with vakues-err,Ok
            //An instance of Result has expect method
            .expect("Failed to read line~");

        //Converting String to Int-32(u32) by trimming and parsing.
        let guess: u32 =match guess.trim().parse(){
            Ok(num) =>num ,
            Err(_) => continue,
        };

        let secret_number=rand::thread_rng().gen_range(1..=10);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Smaller than secret num,You Lost!"),
            Ordering::Greater => println!("Larger than secret num,You Lost!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        //{} used as placeholders
        println!("You guessed: {guess}");
    }
}
