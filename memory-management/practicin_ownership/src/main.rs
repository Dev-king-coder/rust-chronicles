#![allow(unused)]
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

// Code-3
// dst.push(..) requires the W permission
// let largest = .. removes the W permissions on dst.
// Solution-1
fn add_big_strings_1(dst: &mut Vec<String>, src: &[String]) {
    // cloning largest
    let largest: String = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
    // cloning is expensive
}

// Solution-2
// just using the length of the largest string
fn add_big_strings_2(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}

// Code-4
// we have a vector of non-Copy types like String,
// then how do we safely get access to an element of the vector
// Solution-1
fn get_string_1(){
    // avoid taking ownership of string
    // using immutable reference
    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0];
    println!("{s_ref}!");
}

// Solution-2
fn get_string_2(){
    // for ownership
    // clone the data
    let v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v[0].clone();
    s.push('!');
    println!("{s}");
}

// Solution-3
fn get_string_3(){
    let mut v: Vec<String> = vec![String::from("Hello world")];
    // using v.remove(0) removes vector
    let mut s: String = v.remove(0);
    s.push('!');
    println!("{s}");
    assert!(v.len() == 0);
}
