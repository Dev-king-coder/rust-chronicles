#![allow(unused)]
fn main() {
    // Rust's borrow checker does not contain different paths for a[0], a[1], and so on.
    // It uses a single path a[_] that represents all indexes of a.
    // It does this because it cannot always determine the value of an index. 
    let mut a = [0, 1, 2, 3];
    let x = &mut a[0];
    *x += 1;
    println!("{a:?}");


    // #UNSAFE but still safe 
    /**
    let mut b = [0, 1, 2, 3];
    let y = &mut b[0];
    let z = &b[1];
    *y += *z;
    */

    // So a work around is
    // Using standard functions #GOOD
    let mut b = [0, 1, 2, 3];
    let (y, rest) = a.split_first_mut().unwrap();
    let z = &rest[0];
    *y += *z;
            //OR
    // Using unsafe blocks #BAD
    let mut c = [0, 1, 2, 3];
    let m = &mut a[0] as *mut i32;
    let n = &a[1] as *const i32;
    unsafe { *m += *n; } // DO NOT DO THIS unless you know what you're doing!
}