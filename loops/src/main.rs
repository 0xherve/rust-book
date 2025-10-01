fn main() {
    let mut counter = 0;

    let result = loop {
        counter +=1;

        if counter == 10 {
            break counter + 1;
        }
    };

    println!("The counter stopped and the result is at the {result} try.");
}