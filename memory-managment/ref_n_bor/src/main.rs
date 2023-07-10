// Example-1
//  Does not compile
//  One way to make it work is-
//  return g1,g2 from greet in new variables
//  which in turn act as data entries to main variable "s"
// fn main() {
//     let m1 = String::from("Hello");
//     let m2 = String::from("world");
//     greet(m1, m2);
//     // let (m1_again, m2_again) = greet(m1, m2);
//     let s = format!("{} {}", m1, m2); // Error: m1 and m2 are moved
//     // let s = format!("{} {}", m1_again, m2_again);
// }

// fn greet(g1: String, g2: String) {
//     println!("{} {}!", g1, g2);
// }

// Example-2
fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2); // note the ampersands
    let _s = format!("{} {}", m1, m2);
}

fn greet(g1: &String, g2: &String) { // note the ampersands
    println!("{} {}!", g1, g2);
}