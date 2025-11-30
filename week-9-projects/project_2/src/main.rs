use std::fs::File;
use std::io::{Write, Result};

struct Student {
    name: String,
    matric: String,
    department: String,
    level: u32,
}

fn main() -> Result<()> {
    let students = vec![
        Student {
            name: "Oluchi Mordi".to_string(),
            matric: "ACC102111".to_string(),
            department: "Accounting".to_string(),
            level: 300,
        },
        Student {
            name: "Adams Aliyu".to_string(),
            matric: "ECO110018".to_string(),
            department: "Economics".to_string(),
            level: 100,
        },
        Student {
            name: "Shania Bolade".to_string(),
            matric: "CSC103288".to_string(),
            department: "Computer".to_string(),
            level: 200,
        },
        Student {
            name: "Adekunle Gold".to_string(),
            matric: "EEE110202".to_string(),
            department: "Electrical".to_string(),
            level: 200,
        },
        Student {
            name: "Bianca Edemoh".to_string(),
            matric: "MEE102040".to_string(),
            department: "Mechanical".to_string(),
            level: 100,
        },
    ];

    println!("PAU SMIS");
    println!("{:<20} {:<15} {:<15} {:<5}",
        "Student Name", "Matric Number", "Department", "Level");

    for s in &students {
        println!(
            "{:<20} {:<15} {:<15} {:<5}",
            s.name, s.matric, s.department, s.level
        );
    }

    let mut file = File::create("students.txt")?;
    writeln!(file, "PAU SMIS")?;
    writeln!(file, "{:<20} {:<15} {:<15} {:<5}",
        "Student Name", "Matric Number", "Department", "Level")?;

    for s in &students {
        writeln!(
            file,
            "{:<20} {:<15} {:<15} {:<5}",
            s.name, s.matric, s.department, s.level
        )?;
    }

    println!("\nstudents.txt has been created successfully!");

    Ok(())
}