// src/main.rs

mod helper;
mod math_operations;

fn main() {
    // Using the helper module
    helper::greet("Alice");

    // Using the math_operations module
    let a = 10;
    let b = 5;

    println!("Addition: {}", math_operations::add(a, b));
    println!("Subtraction: {}", math_operations::subtract(a, b));
    println!("Multiplication: {}", math_operations::multiply(a, b));

    match math_operations::divide(a, b) {
        Some(result) => println!("Division: {}", result),
        None => println!("Error: Division by zero"),
    }
}

