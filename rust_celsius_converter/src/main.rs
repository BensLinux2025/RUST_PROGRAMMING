fn main() {
    let celsius: f64 = 25.0;
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{}°C is equal to {}°F", celsius, fahrenheit);
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0/5.0) + 32.0
}

