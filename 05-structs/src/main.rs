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

    main_2();
}

fn main_2() {
    // Method Syntax

    // The syntax for methods is the same as for functions. However, methods are different from functions in that they're defined within the context of a struct, and their first parameter is always `self`, which is a reference to the struct that the method is defined on.

    // Area method defined on the Rectangle struct:

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    // ^ The &self is actually short for self: &Self
    // self is an alias for the _type_ that the impl block is for.

    let rect = Rectangle {
        width: 31,
        height: 51,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    // Methods can take ownership of `self`, borrow `self` immutably as done here, or borrow `self` mutably, just as they can any other paramater.

    // ^ We didn't want to take ownership, since we just wanted to read the data in the struct, so we used &self.

    main_3();
}

fn main_3() {
    // We can choose to give a method the same name as one of the struct's fields:

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn width(&self) -> bool {
            self.width > 0
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    // --------------------------------------------------------------------------------------------

    // Automatic Referencing and Dereferencing

    // When you call a method with object.something(), Rust automatically adds in &, &mut, or * so `object` matches the signature of the method. In other words, the following are the same:

    // p1.distance(&p2);
    // (&p1).distance(&p2);

    // The first one looks much cleaner. This automatic referencing behavior works because methods have a clear receiver—the type of self. Given the receiver and name of a method, Rust can figure out definitively whether the method is reading (&self), mutating (&mut self), or consuming (self). The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice.
    main_4();
}

fn main_4() {
    // --------------------------------------------------------------------------------------------

    // Methods with More Parameters

    // We want an isntance of Rectangle to take another instance of Rectangle and return true if the second Rectangle can fit completely within self.

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); // true
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3)); // false

    // We can tell what the type of the parameter will be by looking at the code that calls the method: rect1.can_hold(&rect2) passes in &rect2, which is an immutable borrow to rect2, an instance of Rectangle. This makes sense because we only need to _read_ rect2.

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
    main_5();
}

fn main_5() {
    // Associated Functions

    // All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl. We can define associated functions that don’t have self as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with. We’ve already used one function like this, the String::from function, that’s defined on the String type.

    // Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct. For example, we could provide an associated function that would have one dimension parameter and use that as both width and height, thus making it easier to create a square Rectangle rather than having to specify the same value twice:

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

    // To call this associated function, we use the :: syntax with the struct name:main_4()
    // let sq = Rectangle::square(3);
    // This function is namespaced by the struct: the :: syntax is used for both associated functions and namespaces created by modules.

    main_6();
}

fn main_6() {
    // Multiple impl Blocks

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    // There’s no reason to separate these methods into multiple impl blocks here, but this is valid syntax. We’ll see a case in which multiple impl blocks are useful in Chapter 10, where we discuss generic types and traits.
}

// Summary

// Methods are functions that are defined on a struct or enum. They’re defined with the impl keyword, and are defined within the context of the type they’re defined on.

// Structs let you create custom types that are meaningful for your domain. By using structs, you can keep associated pieces of data connected to each other and name each piece to make your code clear. In impl blocks, you can define functions that are associated with your type, and methods are a kind of associated function that let you specify the behavior that instances of your structs have.
