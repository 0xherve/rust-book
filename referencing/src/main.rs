fn main() {
    let mut s = String::from("not");

    view(&mut s);
}

fn view(str: &mut String) {
    println!("The string is '{str}'");
    str.push_str(" him");
    println!("The new string is '{str}'");
}