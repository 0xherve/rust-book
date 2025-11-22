//tried the vanilla rust thing
//no Cargo for the function topic to remind me what we are a doing

fn main() {
    let x = plus_one(5);

    println!("the value of x is {x}")
}

fn plus_one(var:u32) -> u32 {
    var+1
}