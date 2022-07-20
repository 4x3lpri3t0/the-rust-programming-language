pub mod c10 {
    use std::fmt::Display;

    pub fn generics_traits_lifetimes() {
        println!("*** Chapter 10 - Generic Types, Traits, and Lifetimes ***");

        // Functions: Avoid code duplication
        find_largest();
        find_largest_refactored();

        // Generic data types
        generics_in_functions();
        generics_in_structs();
        generics_in_enums();
        generics_in_methods();
        generics_in_methods_2();

        // Traits: Defining shared behavior
        traits_definition();
        traits_implementation();
        traits_default_behavior();
        traits_as_params();
        traits_bound_syntax_sugar();
        traits_plus();
        traits_bounds_with_where_clauses();
        traits_largest_function_with_trait_bounds();
        trait_bounds_to_conditionally_implement_methods();

        // Lifetimes: Validating references
        lifetimes_to_prevent_dangling_refs();
        generic_lifetimes_in_functions();
        lifetime_annotation_syntax();
        lifetime_annotations_in_function_signatures();
    }

    fn find_largest() {
        let number_list = vec![34, 50, 25, 100, 65];
        let mut largest = number_list[0];
        for number in number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is {}", largest);
    }

    fn find_largest_refactored() {
        fn largest(list: &[i32]) -> i32 {
            let mut largest = list[0];
            for &item in list.iter() {
                if item > largest {
                    largest = item;
                }
            }

            largest
        }

        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest(&number_list);
        println!("The largest number is {}", result);

        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
        let result = largest(&number_list);
        println!("The largest number is {}", result);
    }

    fn generics_in_functions() {
        // fn largest<T>(list: &[T]) -> T {
        //     let mut largest = list[0];

        //     for &item in list.iter() {
        //         if item > largest {
        //             largest = item;
        //         }
        //     }

        //     largest
        // }

        // ^ doesn't compile yet
        // The body of `largest` won't work for all possible types that T could be.
        // (missing the std::cmp::PartialOrd trait)

        fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
            let mut largest = list[0];

            for &item in list.iter() {
                if item > largest {
                    largest = item;
                }
            }

            largest
        }
    }

    fn generics_in_structs() {
        struct Point<T> {
            x: T,
            y: T,
        }

        struct PointMixedTypes<T, U> {
            x: T,
            y: U,
        }
    }

    fn generics_in_enums() {
        enum Option<T> {
            Some(T),
            None,
        }

        // Enums can use multiple generic types as well.
        enum Result<T, E> {
            Ok(T),
            Err(E),
        }
    }

    fn generics_in_methods() {
        struct Point<T> {
            x: T,
            y: T,
        }

        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }

        let p = Point { x: 5, y: 10 };

        println!("p.x = {}", p.x());

        // We could implement methods only on Point<f32> instances rather than on Point<T>
        // instances with a generic type.
        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }
    }

    fn generics_in_methods_2() {
        // Generic type params in a struct definition aren't always
        // the same as those you use in that struct's method signatures.
        struct Point<T, U> {
            x: T,
            y: U,
        }

        impl<T, U> Point<T, U> {
            fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }

        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c' };
        let p3 = p1.mixup(p2);
        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }

    fn traits_definition() {
        // Traits are like interfaces in other languages, but with some differences...__rust_force_expr!

        // Defining a Trait
        // Trait definitions are a way to group method signatures together to define a set of behaviors
        // necessary to accomplish some purpose.
        pub trait Summary {
            fn summarize(&self) -> String;
        }

        // The compiler will enforce that any type that has the Summary trait will have
        // the method `summarize` defined with this signature exactly.
    }

    fn traits_implementation() {
        pub trait Summary {
            fn summarize(&self) -> String;
        }
        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String,
        }

        impl Summary for NewsArticle {
            fn summarize(&self) -> String {
                format!("{}, by {} ({})", self.headline, self.author, self.location)
            }
        }

        pub struct Tweet {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub retweet: bool,
        }

        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }

        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());
    }

    fn traits_default_behavior() {
        // // Sometimes it's useful to have default behavior for some/all of the methods in a trait.
        // // Then we can keep or verride each method's default behavior.
        // pub trait Summary {
        //     fn summarize(&self) -> String {
        //         String::from("(Read more...)")
        //     }
        // }

        // Default implementations can call other methods in the same trait,
        // even if those other methods don't have a default implementation.
        pub trait Summary {
            fn summarize_author(&self) -> String;

            fn summarize(&self) -> String {
                format!("(Read more from {}...)", self.summarize_author())
            }
        }

        // To use that version of Summary, we only need to define `summarize_author` when we implement the trait on a type:
        impl Summary for Tweet {
            fn summarize_author(&self) -> String {
                format!("@{}", self.username)
            }
        }
        pub struct Tweet {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub retweet: bool,
        }

        let tweet = Tweet {
            username: String::from("le_pinata_destroyer"),
            content: String::from("yadda yadda I'm so smart, look at my witty tweet"),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());
    }

    fn traits_as_params() {
        pub trait Summary {
            fn summarize_author(&self) -> String;

            fn summarize(&self) -> String {
                format!("(Read more from {}...)", self.summarize_author())
            }
        }

        // Using traits to define functions that accept many different types.
        pub fn notify(item: impl Summary) {
            //                ^
            // Instead of a concrete type for the item param, we specify the `impl` keyword and the trait name.
            // This param accepts any type that implements the specified trait.
            println!("Breaking news! {}", item.summarize());
        }
    }

    fn traits_bound_syntax_sugar() {
        pub trait Summary {
            fn summarize_author(&self) -> String;

            fn summarize(&self) -> String {
                format!("(Read more from {}...)", self.summarize_author())
            }
        }
        // `impl` works for straightforward cases.
        //   ^    it's syntax sugar for a longer form: the _trait bound_
        pub fn notify<T: Summary>(item: T) {
            println!("Breaking news! {}", item.summarize());
        }
        // `impl` Trait syntax is convenient in simple cases.
        // Trait bound syuntax can express more complexity in other cases.
        // E.g.: We can have two params that implement Summary. Using the `impl` syntax, we can write:
        pub fn notify2(item1: impl Summary, item2: impl Summary) {
            println!(
                "Breaking news! {} and {}",
                item1.summarize(),
                item2.summarize()
            );
        }
        // If we instead wanted to force both params to have the same type, that's only possible using a trait bound:
        // pub fn notify<T: Summary>(item1: T, item2: T) {}
    }

    fn traits_plus() {
        // To specify more than one trait bound, use `+`:
        // pub fn notify(item: impl Summary + Display) {
        //     // ...
        // }
    }

    fn traits_bounds_with_where_clauses() {
        // Similar to C#, Rust has alternate syntax for specifying trait bounds inside a `where` clause after the function signature.
        // Instead of:
        // fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}
        // We can use a `where` clause:
        // fn some_function<T, U>(t: T, u: U) -> i32
        // where
        //     T: Display + Clone,
        //     U: Clone + Debug,
        // {
        //     ...
        // }
    }

    fn traits_largest_function_with_trait_bounds() {
        fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
            let mut largest = list[0];

            for &item in list.iter() {
                if item > largest {
                    largest = item;
                }
            }

            largest
        }

        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest(&number_list);
        println!("The largest number is {}", result);
        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest(&char_list);
        println!("The largest char is {}", result);
    }
    // ^ If we don't want to restrict the largest function to the types that implement the Copy trait, we could specify that
    // T has the trait bound Clone instead of Copy. Then we could clone each value in the slice when we want the `largest`
    // function to have ownership.
    // Heap allocations can be slow if we're working with large amounts of data, though.
    // If we change the return type to &T instead of T, thereby changing the body of the function to return a reference,
    // we wouldn't need the Clone or Copy trait bounds and we could avoid heap allocations.

    fn trait_bounds_to_conditionally_implement_methods() {
        // By using a trait bound with an impl block that uses generic type params, we can implement methods conditionally
        // for types that implement the specified traits.
        struct Pair<T> {
            x: T,
            y: T,
        }

        // Generic
        impl<T> Pair<T> {
            fn new(x: T, y: T) -> Self {
                Self { x, y }
            }
        }
        // Generic bound by traits
        impl<T: Display + PartialOrd> Pair<T> {
            fn cmp_display(&self) {
                if self.x >= self.y {
                    if self.x >= self.y {
                        println!("The largest member is x = {}", self.x);
                    } else {
                        println!("The largest member is y = {}", self.y);
                    }
                }
            }
        }
        // ^ Conditionally implement methods on a generic type depending on trait bounds.
    }

    fn lifetimes_to_prevent_dangling_refs() {
        // The main aim of lifetimes is to prevent dangling references, which cause
        // a program to reference data other than the data it's intended to reference.

        // E.g. an attempt to use a ref whose value has gone out of scope:
        // {
        //     let r;
        //     {
        //         let x = 5;
        //         r = &x; // [E0597] `x` does not live long enough.
        //     } // <- `x` droppped there while still borrowed

        //     println!("r: {}", r);
        // }

        // That code won't compile because the value `r` is referring to has gone out of scope before we try to use it.
        // `r` lives longer than `x`
        // Rust uses a "borrow checker" to determine that this code is invalid.

        // Borrow checker: compares scopes to determine whether all borrows are valid.
        // The data should have >= lifetime than the reference pointing to it.
    }

    fn generic_lifetimes_in_functions() {
        // If we try to implement the longest function like this, it won't compile:
        // fn longest(x: &str, y: &str) -> &str {
        //     if x.len() > y.len() {
        //         x
        //     } else {
        //         y
        //     }
        // }

        // ^ Compilation error: "missing lifetime specifier"
        //      "this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`"

        // Rust can't tell whether the reference being returned refers to x or y.
        // The if block on the body of this function returns a reference to x and the else block returns a reference to y.
        // The borrow checker doesn't know how the lifetimes of x and y relate to the lifetime of the return value.

        // ****************************************************
        // To fix this, we'll add generic lifetime params
        // that define the relationship between the references.
        // ****************************************************
    }

    fn lifetime_annotation_syntax() {
        // Functions can accept references with any lifetime by specifying a generic lifetime parameter.

        // Examples:
        // &i32        // a reference
        // &'a i32     // a reference with an explicit lifetime
        // &'a mut i32 // a mutable reference with an explicit lifetime
    }

    fn lifetime_annotations_in_function_signatures() {
        // All the refs in the params and the return value must have the same lifetime.
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            // ^ both params are string slices, so they have the same lifetime

            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        println!("Longest: {}", longest("1", "22"));

        // When we specify the lifetime params in this function signature,
        // we're not changing the lifetimes of any values passed in or returned...
        // we're specifying that the borrow checker should reject any values
        // that don't adhere to these constraints.

        // The `longest` function doesn't need to know exactly how long x and y will live,
        // only that some scope can be substituted for 'a that will satisfy this signature.

        // TODO: Continue...
    }
}
