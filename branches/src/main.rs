use std::io;

fn main () {
    let mut number = String::new();

    println!("Enter a number:");
    io::stdin().read_line(&mut number).expect("");

    let x:u32 = number.trim().parse().expect("Enter a valid number");
    
    if x == 5 {
        println!("The number is 5");
    }

    else {
        println!("the number is not 5");
    }
}