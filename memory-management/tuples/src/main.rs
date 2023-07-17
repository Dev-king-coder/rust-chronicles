fn main() {
    let mut name = (
        String::from("Dev"), 
        String::from("Rustacean")
    );
    // let first = &name.0; #SAFE
    let first = get_first(&name);
    name.1.push_str(", Keshwani");
    println!("{first} {}", name.1);
}

// Example how rust loses name paths #UNSAFE
fn get_first(name: &(String, String)) -> &String {
    &name.0
}
// That's strange, since the program was safe before
// we edited it. The edit we made doesn't meaningfully change the runtime behavior. 
// So why does it matter that we put &name.0 into a function

// RUST fears that get_first() might have a reference to name.0,name.1 etc
// So it makes sure that name is not mutated before get_first() is called