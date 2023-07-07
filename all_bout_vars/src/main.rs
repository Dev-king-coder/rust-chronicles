fn main() {
    // mut term makes our vars mutale(changeble)
    let mut x = 5;
    println!("x={x}");
    x = 6;
    println!("x={x}");

    // constants by const keyword
    // nmain convention=> UPPER_CASE
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");

    // let keyword is used for shadowing
    // its different from mut keyword 
    // we cant change data type of var which is mut
    let spaces="  ";
    let spaces=spaces.len();
    println!("{spaces}");
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}
