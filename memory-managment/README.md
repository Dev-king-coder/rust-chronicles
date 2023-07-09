## Ownership
It ensures the safety of memory by having only one owner at a time. Discipline for safely using memory within that way of thinking.

## Goal of Rust

- Program Safety: To ensure that our programs never have a undefined behavior
- Memory Safety: To ensure that our programs never access invalid memory
- Prevent undefind behaviour at compile time: Avoiding bugs at runtime
- Catching bugs at compile-time means fewer runtime checks for those bugs, improving the performance of your software.

How rust has helped in memory safety, check out this [Google Security blog](https://security.googleblog.com/2022/12/memory-safe-languages-in-android-13.html) for details.

## Rust Model of Memory
- Variables lives in stack.
- Stack holds data associated with a specific function.
- Stack is LIFO (Last In First Out) data structure.
- Frames in the stack are associated with a specific function, and are deallocated when the function returns.
``` rust
fn main() {
    let n = 5;
    let y = plus_one(n);
    println!("The value of y is: {y}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```
![Frames and Stack representation](images\stack-code-1.png)
---
- Heap is a memory pool.
- All heap data must be owned by exactly one variable.
- As copying data can take up a lot of memory.
- It holds data that can outlive a function.
- Data on the heap can live indefinitely
- They are allowed to contain pointers.
```rust
fn main() {
    let a = Box::new([0; 1_000_000]);
    let b = a;
}
```
![Heap and Box representation](images\heap-code-2.png)
---

## Principles of Ownership
> Box deallocation principle: If a variable owns a box, when Rust deallocates the variable's frame, then Rust deallocates the box's heap memory.

```rust
// EXAMPLE-1
fn main() {
    let a = Box::new([0; 1_000_000]);
    let b = a;
}
/**
 * 1. When a is bound to Box::new([0; 1_000_000]), we say that a owns the box.
 * 2. When b is bound to a, we say that b takes ownership of the box.
 * 3. Therefore when the scope ends, Rust deallocates the box only once on behalf of b, not a.
 */
```
---

> Moved heap data principle: if a variable x moves ownership of heap data to another variable y, then x cannot be used after the move.

```rust
fn main() {
    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{full}");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    return name
}
/**
 * 1. The string "Ferris" allocated on the heap. Owned by first.
 * 2. Function add_suffix(first) called. Moves ownership first to name. The string data is not copied, but the pointer to the data is copied.
 * 3. Function name.push_str(" Jr.") resizes the string's heap allocation.
 * -> It creates a new larger allocation.
 * -> It writes "Ferris Jr." into the new allocation.
 * -> Frees the original heap memory. First is deallocated from memory.
 * 4. The frame for add_suffix is gone. This function returned name, transferring ownership of the string to full.
 */
```
![Moved heap data principle](images\moved-data-code-3.png)

### Note-1:
Variables cannot be used after they move ownership of heap data.
```rust
println!("{full}, originally {first}"); // first is now used here
```
This results in a compile-time error:
```bash
error[E0382]: borrow of moved value: `first`
 --> test.rs:4:35
  |
2 |     let first = String::from("Ferris");
  |         ----- move occurs because `first` has type `String`, which does not implement the `Copy` trait
3 |     let full = add_suffix(first);
  |                           ----- value moved here
4 |     println!("{full}, originally {first}"); // first is now used here
  |                                   ^^^^^ value borrowed here after move
```
### Note-2:
**Cloning** avoids moving heap data.
```rust
fn main() {
    let first = String::from("Ferris");
    let full = add_suffix(first.clone());
    println!("{full}, originally {first}");
}
```
![Cloning](images\cloning-code-4.png)


