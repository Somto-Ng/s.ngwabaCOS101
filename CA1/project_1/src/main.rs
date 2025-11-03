//Rust program to output name and grade

use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();


    println!("\nPlease enter your name.");
    let mut name = String::new();
    io::stdin()
    .read_line(&mut name)
    .expect("Failed to read input");

    println!("Please enter the first test score: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let score1:u32 = input1.trim().parse().expect("Not a valid number");

    println!("Please enter the second test score: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let score2:u32 = input2.trim().parse().expect("Not a valid number");

    println!("Please enter the third test score: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let score3:u32 = input3.trim().parse().expect("Not a valid number");

    println!("Your name is: {}", name);

    let mut average = (input1 + &input2 + &input3)/3;

    println!("Your average is {}", average);

    if average > 100{
        println!("Invalid scores inputted");
    }
    else if average <= 100 && average >=70{
        println!("Your grade is A");
    }
    else if average < 70 && average >= 60{
        println!("Your grade is B");
    }
    else if average < 60 && average >=50{
        println!("Your grade is C");
    }
    else if average < 50 && average >=45{
        println!("Your grade is D");
    }
    else if average < 45 && average >=0{
        println!("Your grade is F");
    }
    else if average < 0{
        println!("Invalid scores inputted");
    }

   
    //  println!("Your score is: {}", score1);


}
