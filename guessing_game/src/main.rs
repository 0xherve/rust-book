use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Guess the Number!");
    let secret_number= rand::thread_rng().gen_range(1..=10);

    loop {
    print!("Please input your guess: ");

    let mut guess = String::new();
    
    io::stdin()
    .read_line(&mut guess)
    .expect("failed to Read line");

    let guess:u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("You guessed: {guess}");
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Your number is Smaller.\n"),
        Ordering::Greater => println!("Your number is Greater.\n"),
        Ordering::Equal => {
            println!("You win!\n");
            break;
        }
    }
    }
    println!("The secret number was: {secret_number}");

}