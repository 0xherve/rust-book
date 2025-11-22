fn main() {
    let mut s = String::from("not");

    change(&mut s);
}

fn change(str: &mut String) {
    println!("The new string is '{str}'");
    str.push_str(" him");
    println!("The new string is '{str}'");
}