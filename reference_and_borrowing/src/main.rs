fn main() {
    let s=String::from("Hellow world!");
    let len= calculate_ln(&s);
    println!("Length of string {s} is {len}");

    let mut s2=String::from("I love");

    let res=calculate_ln_and_mutate(&mut s2);
    println!("Now the s2 becomes {s2} and we can access it and length is {res}")
}
fn calculate_ln(s:&String)->usize{
    let result=s.len();
    result
}
fn calculate_ln_and_mutate(s:&mut String)->usize{
    s.push_str("Rust!!");
    let length= s.len();
    length
}