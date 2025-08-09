use std::io;
fn main() {
    let number: i32;
    println!("Enter a number");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    number = input.trim().parse().expect("Please type a number");

    if number < 5 {
        println!("Less Than 5");
    } else {
        println!("Greater than or Equal to 5");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
