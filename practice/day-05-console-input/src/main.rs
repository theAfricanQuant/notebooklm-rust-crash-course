use std::io;

fn main() {
    println!("What name should I use?");

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line.");

    println!("Hello, {}", name);
}
