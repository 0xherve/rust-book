fn main () {
    println!("This is the main function!");

    another_one("Seems we can pass things around");
}

fn another_one(text:&str) {
    println!("{text}");
}