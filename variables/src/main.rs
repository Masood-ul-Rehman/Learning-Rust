fn main() {
    let age = 32;
    println!("Age is {}", age);
    let mut mutable_age = 32;
    mutable_age = 33;
    println!("Mutable age is {}", mutable_age);
    const THREE_HOURS_IN_SECONDS: u64 = 3 * 60 * 60;
    println!("Three hours in seconds is {}", THREE_HOURS_IN_SECONDS);

    //shadowing
    let age = 32;
    let age = 33;
    println!("Age is {}", age);

    let x: i32 = 10;
    let x: i32 = x + 9;
    println!("x is {}", x);
    {
        let x: i32 = x - 1;
        println!("x is {}", x);
    }
    println!("x is {}", x);
}
