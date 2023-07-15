fn main() {
    // run: Code-1
    return_a_string_1();
    return_a_string_2();
    return_a_string_3();
    return_a_string_4(&mut String::from("Hi"));
    
    // run: Code-2
    let name = vec![String::from("Ferris")];
    let first = &name[0];
    //stringify_name_with_title_1(&name);
    //stringify_name_with_title_2(name);
    stringify_name_with_title_3(&name);
    stringify_name_with_title_4(&name);
    println!("{}", first);
}

// Code-1
// Solution-1
// Removing the Reference
fn return_a_string_1() -> String {
    let s = String::from("Hello world");
    s
}

// Solution-2
// Return string literal directly
// which lives forever using 'static
fn return_a_string_2() -> &'static str {
    "Hello World"
}

// Solution-3
use std::rc::Rc;
fn return_a_string_3() -> Rc<String> {
    let s = Rc::new(String::from("Hello world"));
    Rc::clone(&s)
}

// Solution-4
fn return_a_string_4(output: &mut String) {
    output.replace_range(.., "Hello world");
}

// Code-2
// Solution-1 #BAD
// Functions should not mutate their inputs,
// if the caller would not expect it
/**fn stringify_name_with_title_1(name: &mut Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}*/

// Solution-2
//  It is very rare for Rust functions to take ownership 
// of heap-owning data structures like Vec and String.
/**fn stringify_name_with_title_2(mut name: Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}*/

// Solution-3 #GOOD
fn stringify_name_with_title_3(name: &Vec<String>) -> String {
    let mut name_clone = name.clone();
    name_clone.push(String::from("Esq."));
    let full = name_clone.join(" ");
    full
}

// Solution-4 #GOOD
fn stringify_name_with_title_4(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}