fn main() {
    let x = plus_one(5);
    let y = x + 8;
    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
