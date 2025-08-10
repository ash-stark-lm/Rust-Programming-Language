use std::io;

/*
    Simple user-friendly output	                    "{}"    (Display)
    Debugging or inspecting data	                "{:?}"  (Debug)
    Printing structs or enums during development	"{:?}"
    Printing custom types user output	Implement Display and use "{}"
    You can also use "{:#?}" for pretty-printed, multiline debug output.



*/
fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        true
    } else {
        false
    }
}

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

    let x = 5;
    println!("{:?}", is_even(x));
}
