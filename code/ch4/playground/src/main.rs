fn main() {
    let mut s = String::from("hello world");
    let mut word = first_word_correct(&s);
    s.clear();
    println!("{}", word)
}

fn play0() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s)
}

fn play1() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s2)
}

fn play2() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn play3() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("{},{}", s1, len)
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_word(s: &String) -> String {
    let bytes = s.as_bytes();
    let mut first_word = String::from("");
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return first_word;
        } else {
            first_word.push(item as char);
        }
    }
    first_word
}

fn first_word_correct(mut s: &String) ->  &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

