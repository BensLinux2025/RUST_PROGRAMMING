// src/main.rs

mod helper;

fn main() {
    let num1 = 10;
    let num2 = 5;

    let sum = helper::add(num1, num2);
    let difference = helper::subtract(num1, num2);

    println!("The sum of {} and {} is {}", num1, num2, sum);
    println!("The difference between {} and {} is {}", num1, num2, difference);
}

