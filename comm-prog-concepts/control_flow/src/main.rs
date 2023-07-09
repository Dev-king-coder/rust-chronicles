fn main() {
    // unlike other langs rust do not convert non-bools to bools
    // if 3{
    //     println!("Hi");
    // }

    // Using else if   
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if expression in let statement
    let number = if true { 5 } else { 6 };
    println!("The value of number is: {number}");
}
