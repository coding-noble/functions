#![deny(clippy::all)]

/*
Nothing was massively different from C# for this one.
*/

// Function definition
// fn is the function key word.
// The name of the parameter comes before what type it is.
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// Function with a return value
// Use the small arrow (->) to show what type of value it is returning.
// Do not need to use the return key word.
fn add_five(a: i32) -> i32 {
    a + 5
}

// Function with multiple parameters and a return value
fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn main() {
    // Calling the greet function
    greet("John");

    // Calling the add_numbers function
    let sum = add_five(3);
    println!("Sum: {}", sum);

    // Calling the multiply function
    println!("Product: {}", multiply(2.5, 3.0));
}
