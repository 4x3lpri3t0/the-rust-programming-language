pub mod c07 {
    pub fn packages_crates_modules() {
        println!("*** Chapter 7 - Packages, Crates, Modules ***");

        // Packages and Crates
        pkg_crates();

        // Modules to Control Scope and Privacy
        modules();

        // Paths for Referring to an Item in the Module Tree
        paths();

        // Starting Relative Paths with Super
        // super::serve_order();
        // -> Goes to parent module.
        // -> Have fewer places to update code in the future if it gets moved to a different module.
    }

    fn pkg_crates() {
        // A _package_ is one or more crates that provide a set of functionalities.

        // Rules:
        // * A package must contain zero or one LIBRARY crates, and no more.
        // * It can contain as many BINARY crates as you’d like, but...
        // * It must contain at least one crate (either library or binary).

        // Creating a new project:
        // $ cargo new <pkg-name>

        // If a pkg contains src/main.rs and src/lib.rs, it has two crates:
        // a library and a binary, both with the same name as the pkg.
    }

    fn modules() {
        // Modules let us organize code within a crate into groups for readibility and easy reuse.
        // -> also control the privacy (private/public) of the code.

        // Creating a new library called restaurant:
        // $ cargo new restaurant --lib

        mod front_of_house {
            mod hosting {
                fn add_to_waitlist() {}
                fn seat_at_table() {}
            }

            mod serving {
                fn take_order() {}
                fn serve_order() {}
                fn take_payment() {}
            }
        }
        // ^ A front_of_house module containing other modules that then contain functions.
        // crate
        //  |> front_of_house
        //          |> hosting
        //              |> (...)
        //          |> serving
        //              |> (...)
    }

    fn paths() {
        // pub fn eat_at_restaurant() {
        //     // Absolute path
        //     crate::front_of_house::hosting::add_to_waitlist();
        //     // Relative path
        //     front_of_house::hosting::add_to_waitlist();
        // }
    }
}
