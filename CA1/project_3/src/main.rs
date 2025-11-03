//Rust program to calculate cost

use std::io;

fn main() {
    println!("Laptops are sold for ₦550,000 with a code L");
    println!("Monitors are sold for ₦120,000 with a code M");
    println!("Keyboards are sold for ₦15,000 with a code K");
    println!("Headsets are sold for ₦25,000 with a code H");

    println!("Enter an item code: ");
    let mut code = String::new();
    io::stdin()
    .read_line(&mut code)
    .expect("Failed to read input");
}
