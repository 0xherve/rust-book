fn main() {
    let s = String::from("not Him");
    let len= calculate_len(&s);
    println!("The length of'{s}' is {len}");
}

fn calculate_len(str: &String) -> usize {
    str.len()
}
