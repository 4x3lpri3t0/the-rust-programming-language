pub mod c09 {
    use std::fs;
    use std::fs::File;
    use std::io;
    use std::io::ErrorKind;
    use std::io::Read;

    pub fn error_handling() {
        // Using panic! and backtrace
        panic();

        // Recoverable errors with Result
        // result();

        // Matching on different errors
        // result_error_match();

        // (Chapter 13 will go deeper into closures)
        // unwrap_or_else();

        // Shortcuts for panic on error: unwrap and expect
        // expect();

        // Using ? operator for propagating errors
        question_mark();
    }

    fn panic() {
        // panic!("crash and burn");

        let vec = vec![1, 2, 3];

        // vec[99]; // panic! (index out of bounds)

        // To enable the backtrace:
        // $ RUST_BACKTRACE=1 cargo run
    }

    fn result() {
        let f = File::open("hello.txt");
        // Use a a match expression to handle the Result variants
        let f = match f {
            Ok(file) => file,
            Err(error) => {
                panic!("Probles opening the file {:?}", error);
            }
        };
    }

    fn result_error_match() {
        let f = File::open("hello.txt");

        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => panic!("Problem opening the file: {:?}", other_error),
            },
        };
    }

    fn unwrap_or_else() {
        let f = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
    }

    fn expect() {
        // Unwrap
        // let f = File::open("hello.txt").unwrap();
        // ^ If we run this code without a hello.txt file, it throws an error message from the panic! call that the unwrap method makes

        // Expect
        let f = File::open("hello.txt").expect("Failed to open hello.txt");
    }

    fn question_mark() {
        fn read_username_from_file() -> Result<String, io::Error> {
            // If the value is an Err, the Err will be returned from the whole function
            // as if we had used the return keyword so the error value gets propagated
            // to the calling code.

            let mut f = File::open("hello.txt")?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(s)

            // Error values that have the ? operator called on them go through the from function, defined in the From trait in the standard library, which is used to convert errors from one type into another.

            // As long as each error type implements the from function to define how to convert itself to the returned error type, the ? operator takes care of the conversion automatically.
        }

        // We can shorten that code further:

        fn read_username_from_file_2() -> Result<String, io::Error> {
            let mut s = String::new();
            File::open("hello.txt")?.read_to_string(&mut s)?; // Chaining method calls w/ ?
            Ok(s)
        }

        // Even more concise:
        fn read_username_from_file_3() -> Result<String, io::Error> {
            fs::read_to_string("hello.txt")
        }
        // ^ Using fs::read_to_string instead of opening and then reading the file.

        // The ? operator can only be used in functions that return a Result.
    }
}
