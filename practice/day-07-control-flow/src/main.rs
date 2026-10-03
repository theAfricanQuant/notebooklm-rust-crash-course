use std::io;

fn main() {
    println!("How many days have you practised Rust?");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read your answer.");

    let practice_days: i32 = input
        .trim()
        .parse()
        .expect("Please enter a whole number, such as 6.");

    let has_foundation = practice_days >= 7;
    let is_positive = practice_days > 0;

    if !is_positive {
        println!("Start with one small program today. Every Rust session counts.");
    } else if has_foundation && practice_days < 30 {
        println!("You have a foundation. Keep building it one focused day at a time.");
    } else if practice_days >= 30 {
        println!("You have real momentum. Keep using the compiler as your coach.");
    } else {
        println!("You are building momentum. Run a small Rust program again tomorrow.");
    }
}
