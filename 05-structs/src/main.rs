fn main() {
    // Defining and Instatiating Structs

    // Structs are more flexible than tuples: you don’t have to rely on the order of the data to specify or access the values of an instance.

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // The struct definition is like a general blueprint for creating instances of the struct.

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // If the instance is mutable, we can change a value by using the dot notation.

    user1.email = String::from("anotheremail@example.com");

    // The entire instance must be mutable; Rust doesn't allow us to mark only certain fields as mutable.

    fn build_user(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            active: true,
            sign_in_count: 1,
        }
    }

    // Using the Field Init Shorthand when Variables and Fields Have the Same Name

    // If the field name and variable name are the same, we can use the shorthand notation to initialize the field.

    fn build_user_2(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    // Creating Instances From Other Instances With Struct Update Syntax

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // ... using the struct update syntax is more concise:

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // Struct update syntax is like assignment with = because it moves the data.
    // We can no longer use user1 after creating user2 because the String in the username field of user1 was moved into user2.
    // If we had given user2 new String values for both email and username, and thus only used the `active` and `sign_in_count` values from user1, then user1 would still be valid after creating user2 (the types of `active` and `sign_in_count` are types that implement the Copy trait, which means that they can be copied by assignment).

    // Using *Tuple Structs* without Named Fields to Create Different Types

    // Tuple structs have the added meaning the struct name provides but don't have names associated with their fields; rather, they just have the types of the fields. They are useful when you want to give the whole tuple a name and make the tuple be a different type from other tuples, and naming each field as in a regular struct would be verbose or redundant.

    // To define a tuple struct, start with the `struct` keyword and the struct name followed by the types in the tuple:

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Each struct is its own type, even though the fields within the struct are all of the same type.

    // Unit-Like Structs Without Any Fields

    // Unit-like structs can be useful in situations in which you need to implement a trait on some type but don't have any data that you want to store in the type itself.

    struct AlwaysEqual;

    let subject = AlwaysEqual;

    // ^ we'll be implementing behavior for this type that every instance is always equal to every instance of every other type, perhaps to have a known result for testing purposes. We wouldn't need any data to implement that behavior.

    // Ownership of Struct Data

    // It's possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes. Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.

    // For now we'll use owned types like String instead of references like &str (no need to worry about lifetimes yet).

    // An Example Program Using Structs

    // let width1 = 30;
    // let height1 = 50;

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(width1, height1)
    // );

    // fn area(width: u32, height: u32) -> u32 {
    //     width * height
    // }

    // Improved version:

    // let rect1 = (30, 50);

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(rect1)
    // );

    // fn area(dimensions: (u32, u32)) -> u32 {
    //     dimensions.0 * dimensions.1
    // }

    // Refactoring with Structs: Adding More Meaning

    // struct Rectangle {
    //     width: u32,
    //     height: u32,
    // }

    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(&rect1)
    // );

    // fn area(rectangle: &Rectangle) -> u32 {
    //     rectangle.width * rectangle.height
    // }

    // ^ this conveys that the width and height are related to each other, and it gives descriptive names to the values rather than using the tuple index values of 0 and 1.

    // Adding Useful Functionality with Derived Traits

    // If we try:
    // println!("rect1 is {}", rect1);
    // We get:
    // error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`

    // ^ structs don't have a provided implementation of Display.

    // Use :? for pretty print strings

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
    // Putting the specifier :? inside the curly brackets tells println! we want to use an output format called Debug.

    // Another way to print out a value using the Debug format is by using the dbg! macro [TODO](Skipping for now)

    // In addition to the Debug trait, Rust has provided a number of traits for us to use with the `derive` attribute that can add useful behavior to our custom types. We'll cover how to implement these traits with custom behavior as well as how to create your own traits in Chapter 10.

    // TODO: Method Syntax (To be continued...)
}
