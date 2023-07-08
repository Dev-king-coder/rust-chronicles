fn test(value: i32, unit: &str) {
    println!("This is a test,x={value}{unit}");
}

fn exp_n_stat() {
    let y = {
        let x = 3;
        // doesn’t have a semicolon at the end
        // Expressions do bote include ";"
        // adding ";" makes it statement
        // as statement won't return a value so it can give error
        x + 1
    };
    println!("The value of y is: {y}");
}

fn returnfn(name: &str) -> String {
    // format!=>Creates a String using interpolation of runtime expressions.
    return format!("Hi {}", name);
}

fn main() {
    println!("Hello, world!");
    test(10, "cm");
    exp_n_stat();
    println!("{}", returnfn("Neeti"));
}
