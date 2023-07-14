fn main() {

    // Example-1
    let mut vec: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &vec[2];
    println!("Third element is {}", *num);
    println!("Again, the third element is {}", *num);
    vec.push(4);
    // println!("Again, the third element is {}", *num);
        // throws an error as vec is mutated
        // and the reference is still alive
        // vec loses its RWO permissions
        // and starts pointing to a new memory location
        // the underlying issue is that num could be invalidated by push.
}