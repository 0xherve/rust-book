use std::io;

fn main () {

    println!("This is the if statement learning file");

    loop {
    let mut number = String::new();

    println!("Enter a number:");
    io::stdin().read_line(&mut number).expect("");

    let x:u32 = match number.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("please enter a valid number.\n");
            continue;
        }
    };

        if x == 5 {
        println!("Congratulations.\nThe number is 5\n");
        break;
    }
    else if x > 5 {
        println!("The number is greater than 5\n");
    }
    else if x < 5 {
        println!("The number is less than 5\n");
    }
    else {
        println!("the number is not 5\n");
    }
    }
    
}