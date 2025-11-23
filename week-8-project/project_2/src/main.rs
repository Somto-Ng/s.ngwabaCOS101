use std::io;

struct Applicant {
    name: String,
    years_exp: u32,
}

fn read_number(prompt: &str) -> u32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<u32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
            }
        }
    }
}

fn read_text(prompt: &str) -> String {
    loop {
        println!("{}", prompt);
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();

        if !trimmed.is_empty() {
            return trimmed.to_string();
        } else {
            println!("Name cannot be empty, try again.");
        }
    }
}

fn main() {
    let count = read_number("How many applicants are being interviewed?");
    let mut applicants: Vec<Applicant> = Vec::new();

    for i in 1..=count {
        println!("\nApplicant {}", i);

        let name = read_text("Enter applicant name:");
        let years_exp = read_number("Enter years of programming experience:");

        applicants.push(Applicant { name, years_exp });
    }

    let mut top_applicant = &applicants[0];

    for applicant in &applicants {
        if applicant.years_exp > top_applicant.years_exp {
            top_applicant = applicant;
        }
    }

    println!(
        "\nThe applicant with the highest experience is: {} ({} years)",
        top_applicant.name, top_applicant.years_exp
    );
}
