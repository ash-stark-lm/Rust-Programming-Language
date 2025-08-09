//Variables And Mutability
//By default varibales are immutable like const in js

fn main() {
    let x = 5;
    println!("The value of x is {}", x);
    /*
     x=6; //immutable gives error
    println!("The value of x is {}", x);
     */

    let mut y = 5; //mut tells that the value can be modified
    println!("The value of y is {}", y);
    y = 6;
    println!("The value of y is {}", y);

    //varibales vs constants in RUST
    //constants requires -> const NAME: TYPE = VALUE //always immutable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!(
        "The value of THREE_HOURS_IN_SECONDS is {}",
        THREE_HOURS_IN_SECONDS
    );

    //Shadowing
    let z = 5;
    println!("The value of z is {}", z);
    let z = 6;
    println!("The value of z is {}", z);
}
