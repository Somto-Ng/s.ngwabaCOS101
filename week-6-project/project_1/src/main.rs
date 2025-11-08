use std::io;

fn main() {
    println!("Welcome to Somto's Restaurant.");
    println!("Here is our menu:");
    println!("P = Poundo Yam/Edinkaiko Soup - ₦3,200");
    println!("F = Fried Rice & Chicken      - ₦3,000");
    println!("A = Amala & Ewedu Soup        - ₦2,500");
    println!("E = Eba & Egusi Soup          - ₦2,000");
    println!("W = White Rice & Stew         - ₦2,500");

    println!("\nEnter your preferred food code (P, F, A, E, W): ");
    let mut food_code = String::new();
    io::stdin()
        .read_line(&mut food_code)
        .expect("Failed to read input");
    let food_code = food_code.trim().to_uppercase();

    println!("Enter quantity: ");
    let mut qty_str = String::new();
    io::stdin()
        .read_line(&mut qty_str)
        .expect("Failed to read input");

    let quantity: i32 = match qty_str.trim().parse() {
        Ok(num) if num > 0 => num, // valid positive number
        _ => {
            println!("Invalid quantity given.");
            return;
        }
    };

    let price = match food_code.as_str() {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _ => {
            println!("Invalid food code entered.");
            return;
        }
    };

    let mut total = price * quantity;

    if total > 10000 {
        let discount = (total as f32) * 0.05;
        total -= discount as i32;
        println!("\nWow, you're a big back and a big spender. A 5% discount has been applied to your bill.");
    }

    println!("Food code: {}", food_code);
    println!("Quantity: {}", quantity);
    println!("Total amount to pay: ₦{}", total);
}
