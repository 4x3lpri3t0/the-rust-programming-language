fn main() {
    println!("*** 4 - OWNERSHIP ***");
    // let mut guess = String::new();

    // Ownership enables Rust to make memory safety guarantees without needing a garbage collector.
    // Memory is managed through a system of ownership with a set of rules that the compiler checks at compile time.
    // Managing heap data is why ownership exists.

    // Ownership Rules:
    // 1- Each value in Rust has a variable that’s called its owner.
    // 2- There can only be one owner at a time.
    // 3- When the owner goes out of scope, the value will be dropped.

    // A variable is valid from the point at which it’s declared until the end of the current _scope_.

    // The types covered previously are all stored on the stack and popped off the stack when their scope is over.
    // We want to look at data that is stored on the heap and explore how Rust knows when to clean up that data, hence...

    // The String Type:

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    // Memory and Allocation:

    let _s = String::from("hello"); // Actual String, not literal
                                    // When a variable goes out of scope, Rust calls the 'drop' function for us.
                                    // (e.g.: Rust calls drop automatically at the closing curly bracket)

    // --

    // Ways Variables and Data Interact: Move

    // Try to use s1 after s2 is created:
    let s1 = String::from("hello");
    let _s2 = s1;
    // println!("{}, world!", s1);
    // -> Error! ^^ value used here after move
    // ... It invalidates the first variable (Move).
    // Instead of trying to copy the allocated memory, Rust considers s1 to no longer be valid.
    // With only s2 valid, when it goes out of scope, it alone will free the memory.

    // [!!!] Rust will never automatically create "deep" copies of your data.

    //--

    // Ways Variables and Data Interact: Clone

    // Clone: Common method to deeply copy the heap data of the String (not just the stack data).

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // --

    // Stack-Only Data: Copy

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // This works though, because types such as integers that have a known size at compile time
    // are stored entirely on the stack, so copies of the actual values are quick to make.

    // Any group of simple scalar values can be Copy.
    // Nothing that requires allocation or is some form of resource is copy.

    // Some of the types that are Copy:
    // * Integer types (u32)
    // * bool
    // * chat
    // * Floating point types (f64)
    // * Tuples (if they only contain types that are also Copy)

    // --

    // OWNERSHIP and FUNCTIONS

    // TODO ...
}
