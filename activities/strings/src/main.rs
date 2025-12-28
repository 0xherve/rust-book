fn main() {
let s1 = String::from("Hello");
let s2 = String::from("World!");
let s3 = s1.clone() + " " + &s2;

println!("The first two strings");
println!("{s1} {s2}");

println!("The final string is:");
println!("{s3}")
}