//There are two main data type subsets: scalar and compound.
/*Scalar Type:
    1.integers,
    2. floating-point numbers
    3. Booleans
    4. characters.
*/
fn main() {
    //Integer data types --> default :i32
    /* 8-bit	i8	    u8
        16-bit	i16	    u16
        32-bit	i32	    u32
        64-bit	i64	    u64
        128-bit	i128    u128
        arch	isize   usize
    */
    //Scalar Data Types
    let x: i8 = -20;
    println!("The value of x is {}", x);
    let y: u8 = 50;
    println!("The value of y is {}", y);

    //floating variables
    let x = 2.0; // f64
    println!("The value of x is {}", x);
    let a: f32 = -10.5;
    println!("The value of a is {}", a);

    //Boolean

    let a: bool = true; //with type annotation
    println!("The value of a is {}", a);
    let b = true;
    println!("The value of b is {}", b);

    //Character

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("The value of c is {}", c);
    println!("The value of z is {}", z);
    println!("The value of heart_eyed_cat is {}", heart_eyed_cat);

    //Compound Data Types
    //Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

    //Tuple
    let tup1: (i32, f64, u8) = (500, 6.4, 1);
    let tup2 = (500, 6.4, 1);
    let (x, y, z) = tup2;
    println!("The value of y is: {y}");
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0; //we can access using .index
    let six_point_four = x.1;

    println!("{:?}", tup1); //to print entire tuple

    //Array

    let a = [1, 2, 3, 4, 5];
    println!("The value of a[0] is: {}", a[0]);

    let a: [i32; 5] = [1, 2, 3, 4, 5]; //with type and size annotation
    println!("The value of a[1] is: {}", a[1]);
}
