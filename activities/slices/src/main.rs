fn main() {
    let mut s = String::from("hello world");

    let first_word = get_first(&s);
    s.clear();

    println!("The first word is: {first_word}");

}

fn get_first(s: &String) -> &str{
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}