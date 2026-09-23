const SECONDS_PER_MINUTE: u32 = 60;

fn main() {
    let x = 4;
    println!("outer x starts as {x}");

    let x = x + 1;
    println!("after shadowing, x is {x}");

    {
        let x = 2;
        println!("inside this block, x is {x}");
    }

    println!("outside the block, x is still {x}");

    let x = "four";
    println!("a new x can be text: {x}");

    let mut deliveries = 3;
    deliveries = deliveries + 1;
    println!("deliveries after one more: {deliveries}");

    println!("seconds in a minute: {SECONDS_PER_MINUTE}");
}
