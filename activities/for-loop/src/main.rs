fn main() {
    let mut count:u32 = 0;
    let mut cycle:u32 = 0;
    while count < 1000000 {
        count += 1;
    if count % 10 == 0 {
        cycle += 1;
        println!("_____ CYCLE {cycle} _____");
    }

}
}
