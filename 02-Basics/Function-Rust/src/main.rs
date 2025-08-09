fn main() {
    println!("Hello, world!");
    hello();
    print_labeled_measurement(5, 'h');
    a();

    let x = five();
    println!("The value of x is {}", x);

    let b = add_one(5);
    println!("The value of b is {}", b);
}

fn hello() {
    println!("I am Ashish")
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

//StateMents and Expressions
//Statements do not return values. -> let y=(let x=32);throws error

fn a() {
    let x = {
        //expression -> returns a value
        let y = 3;
        y + 1 // no semicolon here, so block returns 4
              //Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.
    };
    println!("The value of x is {}", x);
}

//Function With Return Values

fn five() -> i32 {
    5
}

fn add_one(x: i32) -> i32 {
    x + 1 //when returning the value, we do not need to use semicolon
}
