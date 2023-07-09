fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn main() {
    // Safe Program
    let x = true;
    read(x);

    // Unsafe Program
    read(z);
    let z = true; 
    
}