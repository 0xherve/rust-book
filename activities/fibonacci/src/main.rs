use std::io;
fn main() {
    println!("WE ARE MAKING A FIBONACCI SEQUENCE!\n");

    println!("Enter the desired number of digits in the fibonacci sequence:");
    let mut input = String::new();

    io::stdin().read_line(& mut input).expect("Please try again");

    let number:u32 = input.trim().parse().expect("Enter a proper number!");

    if number == 0 {
            println!("0");
        }
        else if number == 1 {
            println!("0,1");
        }
        else {
            let mut a = 0;
            let mut b = 1;

            for _count in 1..=number {
                let next = a+b;
                print!("{next}, ");
                a = b;
                b = next;
            }
            print!("\n");
        }
    
}
