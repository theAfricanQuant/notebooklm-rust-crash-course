fn main() {
    let total = add_numbers(16, 7);
    println!("Together: {total}");
    println!("Days left until 30: {}", days_until_goal(total, 30));
}

fn add_numbers(left: i32, right: i32) -> i32 {
    let result = left + right;
    result
}

fn days_until_goal(current_days: i32, goal: i32) -> i32 {
    if current_days >= goal {
        return 0;
    }

    goal - current_days
}
