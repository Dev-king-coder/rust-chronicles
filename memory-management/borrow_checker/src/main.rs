fn main() {

    // Example-1-shared reference(immutable reference)
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

    
    // Example-2-unique reference(mutable reference)
    let mut vec: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut vec[2];
    *num += 1;
    println!("Third element is {}", *num);
    println!("Vector is now {:?}", vec);
    // Mutable references allow mutation but prevent aliasing
    // Borrowed path vec becomes temporarily unusable, so effectively not an alias.
    // Usefulness is now *num has W permissions to vec[2].


    // Example-3
    let mut vec: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut vec[2];
    let num2: &i32 = &*num;
    println!("{} {}", *num, *num2);
    // the borrow &*num removes the W permission from *num but not the R permission,
    // so println!(..) can read both *num and *num2.

    let mut s = String::from("hello");
    let s2 = &s;
    let s3 = &mut s;
    s3.push_str(" world");
    //println!("{s2}");
}