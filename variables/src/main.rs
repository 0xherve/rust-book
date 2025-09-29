use std::io;

fn main () {
    let array = [10, 11, 12, 13, 14, 15];

    let mut index = String::new();
    println!("Please enter the index you want to see between 0 and 4");

    io::stdin()
    .read_line(&mut index)
    .expect("Failed to read the line");

    let index:usize = index.trim().parse().expect("Please enter a number:");

    println!("\nThe number at {index} is {}", array[index]);
}