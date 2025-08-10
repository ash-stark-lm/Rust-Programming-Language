fn main() {
    let s1 = String::from("hello"); //Creates a String on the heap.
    //s1 is the owner of that memory
    let len = calculate_length(&s1); //& creates a reference to s1
    //we do not transfer ownership -> We are letting another function borrow the value temporarily

    println!("The length of '{s1}' is {len}.");

    let mut s2 = String::from("hello");
    let _s = String::from("hello");

    mutable_borrowing(&mut s2);
    immutable_borrowing(&s2);

    scope();
    //let x = dangle();
    no_dangle();
}
/*
    You allow someone else to use your value without giving it away.
    While borrowed immutably, the original value can still be read, but not modified by the borrower.
    In this  code, calculate_length borrows s1 to read its length.
*/
fn calculate_length(s: &String) -> usize {
    //s is a reference to the same heap data as s1.
    //No heap copy is made — just a pointer is passed.
    s.len()
}

/*--------------Immutable vs mutable Borrow-----------------*/
//  &String-> immutable borrow read only
// &mut String-> mutable borrow

fn immutable_borrowing(s: &String) {
    //    s.push_str("Immutable"); // ^ `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    println!("{s}");
}

//Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value
/*
  let mut s = String::from("hello");

    let _r1 = &mut s;
    let _r2 = &mut s; //cannot borrow `s` as mutable more than once at a time
    println!("{_r1}, {_r2}");

*/
fn mutable_borrowing(s: &mut String) {
    s.push_str(", world");
    println!("{s}");
}

/*--------------Scope----------------- */

fn scope() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        r1.push_str(" append");
        println!("{r1}");
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    //Now see we can't borrow the same as immutable and mutable at once

    let mut s2 = String::from("New String");
    let t1 = &s2; //no issue --> immutable Borrow
    let t2 = &s2; //No issue --> immutable Borrow
    // let t4 = &mut s2; //cannot borrow `s2` as mutable because it is also borrowed as immutable mutable borrow occurs here
    println!("{t1}, {t2}"); // basically scope of t1 ended here coz iske baad it never used;

    let t3 = &mut s2;
    println!("{t3}");
    // println!("{t1}, {t2}, {t3}"); //Now it will give error---> cannot borrow `s2` as mutable because it is also borrowed as imm
    println!("{r2}");
}

/*---------------Dangling Reference-------------- */

/*
fn dangle() -> &'static String {
    // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String
    &s //gives errr-> we return a reference to the String,
} //see when function gets completeed it will free up its emmory meaning s would get freed so where would ref_to_s points now -> called dangling reference
*/
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
