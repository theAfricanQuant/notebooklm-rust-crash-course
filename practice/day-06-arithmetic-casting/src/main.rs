use std::io;

fn main() {
    println!("How many minutes did you study this week?");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    let weekly_minutes: i64 = input.trim().parse().expect("Please enter a whole number.");
    let days: i64 = 7;

    let whole_daily_average = weekly_minutes / days;
    let remainder = weekly_minutes % days;
    let precise_daily_average = weekly_minutes as f64 / days as f64;

    println!("{weekly_minutes} minutes over {days} days:");
    println!("Whole-number daily average: {whole_daily_average}");
    println!("Precise daily average: {precise_daily_average}");
    println!("Minutes left after equal whole-minute days: {remainder}");
}
