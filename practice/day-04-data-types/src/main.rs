fn main() {
    let jobs: u32 = 4;
    println!("Jobs = {jobs}");

    let price: f64 = 49.95;
    println!("Price is {price}");

    let approved: bool = true;
    println!("approved is {approved}");

    let grade: char = 'A';
    println!("grade = {grade}");

    let customer = ("Müller", 3);
    println!("the customer's Name is {}", customer.0);
    println!("service count is {}", customer.1);

    let services = ["Hedge trimming", "Lawn care", "Cleanup"];

    println!("The first service is {}", services[0]);
}
