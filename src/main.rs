mod test;

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

    // let s = String::from("hello"); // Actual String, not literal
    // When a variable goes out of scope, Rust calls the 'drop' function for us.
    // (e.g.: Rust calls drop automatically at the closing curly bracket)

    // Ways Variables and Data Interact: Move

    // To ensure memory safety...
    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}, world!", s1); // ERROR! ('Borrow of moved value: s1')
    // Instead of trying to copy the allocated memory, Rust considers s1 to no longer be valid and, therefore,
    // doesn’t need to free anything when s1 goes out of scope.

    /*
    The concept of copying the pointer, length, and capacity without copying the data probably sounds like
    making a shallow copy. But because Rust also invalidates the first variable, instead of being called
    a shallow copy, it’s known as a move (in this example s1 was moved into s2).
    */

    // After s1 has been invalidated - With only s2 valid, it alone will
    // free the memory when it goes out of scope.

    // !!!
    // Rust will never automatically create “deep” copies of your data.
    // -> Any automatic copying can be assumed to be inexpensive in terms of runtime performance.
    // !!!

    // Ways Variables and Data Interact: Clone
    // Use the `clone` method for deep copying heap data.
    //

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {} !!!", s1, s2);

    test::test::test();
}
