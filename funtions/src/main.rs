fn main() {
    println!("Funtions Learning!");
    my_function(24);
    let valu = is_odd_even(2);
    println!("number is {}", valu);
}

fn my_function(number: i32) {
    println!("my_function  number is ${}", number);
    let data = add_two_numbers(422, 21);
    print!("additon of two numbers {}", data)
}
fn add_two_numbers(number_one: i32, number_two: i32) -> i32 {
    number_one + number_two
}
fn is_odd_even(a: i32) -> bool {
    if a % 2 == 0 {
        return true;
    } else {
        return false;
    }
}
