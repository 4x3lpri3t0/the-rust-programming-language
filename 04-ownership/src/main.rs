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

    // When a variable that includes data on the heap goes out of scope,
    // the value will be cleaned up by `drop` unless the data has been moved to be owned by another variable.

    // Return multiple values using a tuple:
    let (s2, len) = calculate_length(s1);

    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len();

        (s, length)
    }

    // REFERENCES and BORROWING

    let s_ref = String::from("reference");
    let len = calculate_length_with_reference(&s_ref);

    fn calculate_length_with_reference(s: &String) -> usize {
        s.len() // s <- s1 <- "reference"
    }
    // These ampersands (&) are refs: refer to some value without taking ownership of it.

    // When functions have references as parameters instead of the actual values,
    // we don’t need to return the values in order to give back ownership, because we never had ownership.
    // BORROWING: Having references as function parameters.

    // Borrowed vaues CANNOT be modified... "error[E0596]: cannot borrow immutable borrowed content `*some_string` as mutable"
    // We are not allowed to modify something we have a reference to.

    // MUTABLE REFERENCES

    fn change(some_string: &mut String) {
        some_string.push_str(", world")
    }
    let mut s = String::from("Hello");
    change(&mut s);

    // Mutable ref have a restriction: you can have only one mutable ref to a particular
    // piece of data in a particular scope. Will fail:
    // let r1 = &mut s;
    // let r2 = &mut s;

    // Most languages let you mutate whenever you'd like
    // -> Rust can prevent data races at compile time

    // Use curly braces to create new scope, allowing for multiple mut ref, just not _simultaneous_ ones:
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;

    // Also: Cannot hav a mut ref while we have a an immutable one.

    // DANGLING REFERENCES

    // let ref_to_nothing = dangle();
    // fn dangle() -> &String {
    //     let s = String::from("The compiler won't let me dangle");

    //     &s
    // } // Here s goes out of scope and is dropped. Its memory goes away. Danger!
    // // That returning reference would be pointing to an invalid string.

    // Instead, return the string directly:
    fn no_dangle() -> String {
        let s = String::from("Hello");

        s
    }

    // RULES of REF:
    // * Can have _either_ but not both of:
    //      - 1 mut ref
    //      (or)
    //      - N immutable refs
    // * Refs must always be valid

    // Diff kind of refs: Slices...

    // The SLICE type
    // * Does not have ownership
    // * Lets you ref a contiguous sequence of elements in a collection

    // iter() : Returns each element in a collection.
    // enumerate() : Wraps the result of `iter` and returns each element as part of a tuple instead (idx, &item).
    //  -> We can use patterns to destructure that tuple (like anything else in Rust).

    // STRING SLICES

    // String slice: Ref to a PART of a String.
    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];
    // let slice_of_whole_str = &s[..]

    // Function that returns the first word of a string:
    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b'' {
                return &s[..i];
            }
        }

        &s[..]
    }

    // STRING LITERALES are SLICES
    let s = "Hello, world!";
    // The type of s here is &str: it’s a slice pointing to that specific point of the binary.
    // String literals are immutable; &str is an immutable ref.

    // STRING SLICES as params
    // &str allows us to use the fn for on both String and &str (slice) values.
}
