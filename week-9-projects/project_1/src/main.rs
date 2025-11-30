use std::fs::File;
use std::io::Write;

fn main() {
    let lager = vec![
        "33 Export",
        "Desperados",
        "Goldberg",
        "Gulder",
        "Heineken",
        "Star",
    ];

    let stout = vec![
        "Legend",
        "Turbo King",
        "Williams",
    ];

    let non_alcoholic = vec![
        "Maltina",
        "Amstel Malta",
        "Malta Gold",
        "Fayrouz",
    ];

    let mut file = File::create("drinks.txt").expect("Could not create file");

    writeln!(file, "High-Quality Drink Categories\n").unwrap();

    writeln!(file, "Lager:").unwrap();
    for item in &lager {
        writeln!(file, "- {}", item).unwrap();
    }

    writeln!(file, "\nStout:").unwrap();
    for item in &stout {
        writeln!(file, "- {}", item).unwrap();
    }

    writeln!(file, "\nNon-Alcoholic:").unwrap();
    for item in &non_alcoholic {
        writeln!(file, "- {}", item).unwrap();
    }

    println!("drinks.txt has been created successfully!");
}