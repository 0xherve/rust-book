use std::{io};

fn main() {
    println!("LET'S CONVERT TEMPERATURES!");

    println!("Enter 1 for converting from Fahrenheit to Celsius!");
    println!("Enter 2 for converting from Celsius to Fahrenheit!");

    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Please Try Again!");

    let choice:i8 = answer.trim().parse().expect("Please try again");

    if choice == 1 {
        let mut input = String::new();
        println!("Enter the Temperature to be converted to celsius:");
        io::stdin().read_line(&mut input).expect("Failed!");
        
        let degree:u32 = input.trim().parse().expect("Failed.");
        to_celsius(degree);
    }
    else if choice == 2 {
        let mut input = String::new();
        println!("Enter the Temperature to be converted to Fahrenheit:");
        io::stdin().read_line( &mut input).expect("Failed!");
        
        let degree:u32 = input.trim().parse().expect("Failed.");
        to_fahrenheit(degree);
    }
    else {
        println!("The input wasn't 1 or 2. Please try again!");
        std::process::exit(0);
    }
}

fn to_celsius(deg:u32) {
    let result = (deg - 32)*5/9;
    println!("{deg} in celsius is: {result}");
}

fn to_fahrenheit(deg:u32) {
    let result:u32 = (deg*9/5) + 32;
    println!("{deg} in fahrenheit is: {result}");
}
