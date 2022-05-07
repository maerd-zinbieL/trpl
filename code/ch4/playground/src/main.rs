fn main() {
    play1()
}

fn play0() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s)
}

fn play1() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s1)
}
