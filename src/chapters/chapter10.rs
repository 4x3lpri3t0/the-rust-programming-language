pub mod c10 {
    pub fn generics_traits_lifetimes() {
        println!("*** Chapter 10 - Generic Types, Traits, and Lifetimes ***");

        // Using functions to avoid code duplication
        find_largest();
        find_largest_refactored();

        // Generic Data Types
        generics_in_functions();
        generics_in_structs();
        generics_in_enums();
        generics_in_methods();
        generics_in_methods_2();

        // Traits: Defining Shared Behavior
        defining_a_trait();
        implementing_a_trait();
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

    fn defining_a_trait() {
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

    fn implementing_a_trait() {
        // TODO
    }
}
