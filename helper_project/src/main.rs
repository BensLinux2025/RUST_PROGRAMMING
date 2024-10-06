mod helper;  // Declaring the helper module

fn main() {
    println!("Hello from main!");
    helper::helper_function();  // Calling the public function from helper.rs
}

