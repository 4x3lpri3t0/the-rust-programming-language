fn main() {
    // let mut guess = String::new();
    println!("*** OWNERSHIP ***");

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

    let s = String::from("hello"); // Actual String, not literal
                                   // When a variable goes out of scope, Rust calls the 'drop' function for us.
                                   // (e.g.: Rust calls drop automatically at the closing curly bracket)

    // Ways Variables and Data Interact: Move

    // TODO ...
}
