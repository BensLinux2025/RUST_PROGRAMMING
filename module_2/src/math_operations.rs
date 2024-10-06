// src/math_operations.rs

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn divide(a: i32, b: i32) -> Option<f64> {
    if b != 0 {
        Some(a as f64 / b as f64)
    } else {
        None // Return None if division by zero
    }
}

