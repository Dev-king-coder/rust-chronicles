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
