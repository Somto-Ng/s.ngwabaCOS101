use std::io;
use std::f64::consts::PI;

fn main() {
    println!("=== MTH 101 SHAPES CALCULATOR ===");
    println!("Choose an option:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");
    println!("Enter your choice:");

    let choice = read_input().trim().parse::<u32>().unwrap_or(0);

    match choice {
        1 => area_trapezium(),
        2 => area_rhombus(),
        3 => area_parallelogram(),
        4 => area_cube(),
        5 => volume_cylinder(),
        _ => println!("Invalid choice! Please restart the program."),
    }
}

fn area_trapezium() {
    println!("Enter height:");
    let height = read_input().trim().parse::<f64>().unwrap();

    println!("Enter base1:");
    let base1 = read_input().trim().parse::<f64>().unwrap();

    println!("Enter base2:");
    let base2 = read_input().trim().parse::<f64>().unwrap();

    let area = (height / 2.0) * (base1 + base2);

    println!("Area of Trapezium = {}", area);
}

fn area_rhombus() {
    println!("Enter diagonal1:");
    let d1 = read_input().trim().parse::<f64>().unwrap();

    println!("Enter diagonal2:");
    let d2 = read_input().trim().parse::<f64>().unwrap();

    let area = 0.5 * d1 * d2;

    println!("Area of Rhombus = {}", area);
}

fn area_parallelogram() {
    println!("Enter base:");
    let base = read_input().trim().parse::<f64>().unwrap();

    println!("Enter altitude:");
    let height = read_input().trim().parse::<f64>().unwrap();

    let area = base * height;

    println!("Area of Parallelogram = {}", area);
}

fn area_cube() {
    println!("Enter length of side:");
    let side = read_input().trim().parse::<f64>().unwrap();

    let area = 6.0 * side.powf(2.0);

    println!("Area of Cube = {}", area);
}

fn volume_cylinder() {
    println!("Enter radius:");
    let r = read_input().trim().parse::<f64>().unwrap();

    println!("Enter height:");
    let h = read_input().trim().parse::<f64>().unwrap();

    let volume = PI * r.powf(2.0) * h;

    println!("Volume of Cylinder = {}", volume);
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}
