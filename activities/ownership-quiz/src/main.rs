// PROGRAM 1
// /// Makes a string to separate lines of text, 
// /// returning a default if the provided string is blank
// fn make_separator(user_str: &str) -> &str {
//     if user_str == "" {
//         let default = "=".repeat(10);
//         &default //cannot return a reference to data owned by default 
//     } else {
//         user_str
//     }
// }
// 

//PROGRAM 2
/// Gets the string out of an option if it exists,
/// returning a default otherwise
fn get_or_default(arg: Option<String>) -> String {
    let s:String = arg.unwrap_or(String::from("default"));
    s
}
fn main() {

    println!("Testing the get or default function:");
    let test1 = Some(String::from("Hello, World!"));
    let test2: Option<String> = None;
    
    println!("Test 1: {}", get_or_default(test1));
    println!("Test 2: {}", get_or_default(test2));
}