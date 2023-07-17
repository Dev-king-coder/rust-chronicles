### Note- Check [code file](src\main.rs) for Solutions

## Code-1
> **PROBLEM**: Lifetime of the referred data
```rust
fn return_a_string() -> &String {
    let s = String::from("Hello world");
    &s
}
```
---
## Code-2
> **PROBLEM**: Trying to mutate read-only data, or trying to drop data behind a reference.
```rust
fn stringify_name_with_title(name: &Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}
// ideally: ["Ferris", "Jr."] => "Ferris Jr. Esq."
```
---

## Code-3
> **PROBLEM**: Using a reference to heap data that gets deallocated by another alias.
```rust
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = 
      dst.iter().max_by_key(|s| s.len()).unwrap();
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
}
```
---
## Code-4
- An i32 does not own heap data, so it can be copied without a move.
```rust
fn main() {
    let v: Vec<i32> = vec![0, 1, 2];
    let n_ref: &i32 = &v[0];
    let n: i32 = *n_ref;
}
```
- A String does own heap data, so it can not be copied without a move.
```rust
fn main() {
    let v: Vec<String> = vec![String::from("a"), String::from("b")];
    let s_ref: &String = &v[0];
    let s: String = *s_ref;
}
```