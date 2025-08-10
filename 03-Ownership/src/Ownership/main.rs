fn main() {
    let s = "Hello";
    {
        let _x = "Hi There"; //if we declare it with _x it will not give warning

        //let y = "Bro"; //Unused variable giving warning
    }
    /*
        Type: &'static str (a string slice pointing to data baked into the program binary).
       Stored in read-only memory in the compiled executable
       Immutable — you cannot change it.
       No heap allocation — it’s just a pointer to fixed data.
       Lifetime is 'static → exists for the whole program runtime.
    */

    //println!("{x}"); not in scope
    println!("{s}");

    let mut a = String::from("Hello"); //by default immutable
                                       //Heap allocation for the characters ['H', 'e', 'l', 'l', 'o'].
                                       //a owns that heap memory.
                                       /*
                                           Type: String (heap-allocated, growable text buffer).
                                           Data is copied from "Hello" (the literal) into heap memory at runtime.
                                           You can modify it — push, append, clear, etc.
                                           It owns its contents and will drop() (free) them when it goes out of scope.
                                       */

    println!("A= {a}");
    a.push_str("World");
    a.push('j'); //appending character

    println!("A= {a}");
}

//Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope
//Rust calls drop automatically at the closing curly bracket.
