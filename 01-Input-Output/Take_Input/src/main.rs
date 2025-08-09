use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a number:");

    io::stdin()
        .read_line(&mut input) //The full job of read_line is to take whatever the user types into standard 
        //input and append that into a string (without overwriting its contents), so we therefore pass that string as an argument. 
        //The string argument needs to be mutable so the method can change the string’s content.
        .expect("Failed to read line");

    let num: i32 = input.trim().parse()
        .expect("Please enter a valid number");

    println!("You entered number: {}", num);
}
