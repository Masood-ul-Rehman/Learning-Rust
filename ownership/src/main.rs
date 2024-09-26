fn main() {
    let s: String = String::from("Hello");

    let s1 = s;

    let mut s2: String = s1.clone();

    s2.push_str(" Pushed string");

    println!("s1 is {s1}");
    println!("s2 is {s2}");

    let num = 15;
    let result = add(num);
    println!("this is result {result}");
    let s=gives_ownership();
    println!(" {s}");
    take_ownership_string(s1);
    //this print won't work because ownership of s1 changed when funtion call happened//
    // println!("this is s1 {s1}");
}
fn take_ownership_string(str: String) {
    println!("this is the comming str {str}");
}
fn gives_ownership()->String {
    let s:String=String::from("This is string from gives ownership");
    s
}

fn add(x: i32) -> i32 {
    x + 10
}
