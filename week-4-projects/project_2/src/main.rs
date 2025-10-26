use std::io;

fn main() {
    println!("Is the employee experienced? (yes/no):");
    let mut experience_input = String::new();
    io::stdin().read_line(&mut experience_input).expect("Failed to read input");
    let experience_input = experience_input.trim().to_lowercase();

    println!("Enter the age of the employee:");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("Failed to read input");
    let age: i32 = age_input.trim().parse().expect("Please enter a valid number");

    let incentive: i64;

  
    if experience_input == "yes" {
        if age >= 40 {
            incentive = 1_560_000;
        } else if age >= 30 && age < 40 {
            incentive = 1_480_000;
        } else if age < 28 {
            incentive = 1_300_000;
        } else {
            incentive = 1_000_000;
        }
    } else {
        incentive = 100_000;
    }

    println!("The annual incentive for the employee is ₦{}", incentive);
}