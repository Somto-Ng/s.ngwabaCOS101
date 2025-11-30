use std::fs::File;
use std::io::{Write, Result};

fn main() Result<()> {
    let commissioners = vec![
        "Aigboqun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etiyeye",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    println!("{:<5} {:<30} {:<20} {:<15}",
        "S/N", "Name of Commissioner", "Ministry", "Geo-Political Zone");

    for i in 0..commissioners.len() {
        println!(
            "{:<5} {:<30} {:<20} {:<15}",
            i + 1,
            commissioners[i],
            ministries[i],
            zones[i]
        );
    }

    let mut file = File::create("efcc_ministers.txt")?;
    writeln!(file, "{:<5} {:<30} {:<20} {:<15}",
        "S/N", "Name of Commissioner", "Ministry", "Geo-Political Zone")?;

    for i in 0..commissioners.len() {
        writeln!(
            file,
            "{:<5} {:<30} {:<20} {:<15}",
            i + 1,
            commissioners[i],
            ministries[i],
            zones[i]
        )?;
    }

    println!("\nefcc_ministers.txt created successfully!");

    Ok(())
}