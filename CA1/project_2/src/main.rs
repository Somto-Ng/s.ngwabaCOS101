// Rust program to calculate compound interest given p, r and t

use std::io;

fn main() {
	let mut p = String::new();
	let mut r = String::new();
	let mut t = String::new();

    println!("Enter P: ");
    io::stdin().read_line(&mut p).expect("Not a valid string");
    let x:f32 = p.trim().parse().expect("Not a valid number");


    println!("Enter R: ");
    io::stdin().read_line(&mut r).expect("Not a valid string");
    let y:f32 = r.trim().parse().expect("Not a valid number");


    println!("Enter T: ");
    io::stdin().read_line(&mut t).expect("Not a valid string");
    let z:f32 = t.trim().parse().expect("Not a valid number");

    let a:f32 = p *(1 + r / 100) ^ t;

    println!("The Compound Interest is: {}", a);
}
