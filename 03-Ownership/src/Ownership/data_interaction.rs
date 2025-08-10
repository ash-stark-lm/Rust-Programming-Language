fn main() {
    /*---------------Variables and Data Interacting with Moves---------------*/

    let x = 31;
    let mut y = x; //Rust copies the value of x into y in a new stack slot
    println!("X={x} Y={y}"); //Now see

    y = y + 1;
    println!("X={x} Y={y}"); //x value didn't changed

    //Rust will never automatically create “deep” copies of your data.

    /*A String is made up of three parts, shown on the left:
    - a pointer to the memory that holds the contents of the string,
    - a length,
    - and a capacity.
    - This group of data is stored on the stack.
    */

    //When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack.
    //We do not copy the data on the heap that the pointer refers to

    /*
    Rust automatically calls the drop function and cleans up the heap memory for that variable.
     But  both data pointers pointing to the same location. This is a problem: when s2 and s1 go out of scope,
     they will both try to free the same memory.
     This is known as a double free error and is one of the memory safety bugs we mentioned previously.
      Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.
    To ensure memory safety, after the line let s2 = s1;, Rust considers s1 as no longer valid.
    */

    let s1 = String::from("Hello");
    let s2 = s1;
    //Now it is neither shallow nor deep copy
    // now s2 points to the memory address wher s1 was pointing and s1 is no longer valid=> this is called move
    println!(" s2={s2}"); //we can't call s1 now

    /*---------------Scope and assignments---------------*/

    /*
    The inverse of this is true for the relationship between scoping, ownership, and memory being freed via the drop function as well.
    When you assign a completely new value to an existing variable.
     Rust will call drop and free the original value’s memory immediately.
    */
    let mut _s = String::from("hello");
    _s = String::from("ahoy");
    //We initially declare a variable s and bind it to a String with the value "hello".
    //Then we immediately create a new String with the value "ahoy" and assign it to s.
    // At this point, nothing is referring to the original value on the heap at all.
    //The original string thus immediately goes out of scope. Rust will run the drop function on it and its memory will be freed right away.
    // When we print the value at the end, it will be "ahoy, world!".
    println!("{_s}, world!");

    /*--------------Clone Deep Copy--------------- */
    //If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone
    let mut s1 = String::from("hello");
    let s2 = s1.clone(); //deep copy

    println!("s1 = {s1}, s2 = {s2}");
    s1.push_str(" Brother");
    println!("s1 = {s1}, s2 = {s2}"); //no chnage on s2 as both are now pointing to different memory

    //Now
    let _x = 32;
    let _y = x;
    println!("X={x} Y={y}");
    /*The reason is that types such as integers that have a known size at compile time are stored entirely on the stack,
     so copies of the actual values are quick to make. That means there’s no reason we would want to prevent x from
     being valid after we create the variable y. In other words, there’s no difference between deep and shallow copying here,
    so calling clone wouldn’t do anything different from the usual shallow copying, and we can leave it out.
     */

    /*-----------------------Copy trait-------------------- */
    /*Rust has a special annotation called the Copy  that we can place on types that are stored on the stack, as integers are .
     If a type implements the Copy trait, variables that use it do not move,
     but rather are trivially copied, making them still valid after assignment to another variable.

     some of the types that implement Copy
    All the integer types, such as u32.
    The Boolean type, bool, with values true and false.
    All the floating-point types, such as f64.
    The character type, char.
    Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.


    */
}
