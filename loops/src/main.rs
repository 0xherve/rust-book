fn main() {
    let mut count = 0;
    'counting: loop {
        println!("\nWe are on loop {count}");
        let mut number = 3;

        loop {
            if number == 1 {
                println!("Cycle finished ---------------- ");
                break;
            }
            if count == 2 {
                println!("\nWe are done with all the loops.\n");
                break 'counting;
            }

            println!("Number {number}");
            number -=1 ;
        }
        count += 1;
    } 
    println!("----------------------------------------------------------");
}