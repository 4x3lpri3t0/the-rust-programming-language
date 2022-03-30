pub mod c03 {
    pub fn common_concepts() {
        println!("*** Chapter 3 - Common Concepts ***");

        // Shadowing
        shadowing();

        // Data Types
        data_types();

        // Functions
        functions();

        // Control Flow
        control_flow();
    }

    fn shadowing() {
        // We can _shadow_ a variable by using the same variable’s name
        // and repeating the use of the let keyword as follows:
        let x = 5;
        let x = x + 1;
        let x = x * 2;
        println!("The value of x is {}", x);

        // Shadowing spares us from having to come up with different names,
        // such as spaces_str and spaces_num; instead, we can reuse the same name:
        let spaces = "    "; // str
        let spaces = spaces.len(); // usize
        println!("There are {} spaces in there", spaces);

        // However, if we try to use mut for this, as shown here, we’ll get a compile-time error:
        // let mut spaces = "   ";
        // spaces = spaces.len();
    }

    fn data_types() {
        // Rust -> Statically typed (must know types of all variables at compile time)

        // Scalar Types -> Single value (int, float, bool, char)
        // isize and usize types depend on the arch of the computer your program is running on:
        // 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

        // Other int representations (Literals):

        // Decimal	98_222
        // Hex	0xff
        // Octal	0o77
        // Binary	0b1111_0000
        // Byte (u8 only)	b'A'

        // integer types default to i32: this type is generally the fastest, even on 64-bit systems

        // Floating-Point types:

        let f1 = 2.0; // f64 (default)
                      // let f2: f32 = 3.0; // f32

        fn print_type_of<T>(_: &T) {
            println!("{}", std::any::type_name::<T>())
        }

        println!("f1 type:");
        print_type_of(&f1); // f64 (default)

        // Char:

        // let heart_eyed_cat = '😻';
        // Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means
        // it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters

        // ==========================================================================

        // Compound Types -> Can group multiple values into one type (tuples, arrays)

        // Tuple:

        // Tuples have a fixed length: once declared, they cannot grow or shrink in size.

        let tup: (i32, f64, u8) = (500, 6.4, 1); // Each position in the tuple has a type
        println!("tup.0 type:");
        print_type_of(&tup.0);
        // The variable tup binds to the entire tuple, because a tuple is considered a single compound element.
        // We can use pattern matching to destructure a tuple value:
        let (_, y, _) = tup; // _destructuring_
        println!("The value of y is: {}", y);

        // Array:

        let a = [1, 2, 3, 4, 5]; // Allocated in stack (not heap)
        let a: [i32; 5] = [1, 2, 3, 4, 5];
        let a = [3; 5]; // [3, 3, 3, 3, 3]
        println!("Value of first element in `a`: {}", a[0]);
    }

    fn functions() {
        fn another_function() {
            println!("Another function.");
            another_function_with_params(42);
        }

        fn another_function_with_params(x: i32) {
            println!("The answer to life the universe and everything is {}", x);
        }

        another_function();
        expression(); // Calling a function before declaring it.

        fn expression() {
            let x = 5;

            let y = {
                let x = 3;
                x + 1 // returns 4
                      // !!! EXPRESSIONS DO NOT INCLUDE SEMICOLONS !!!
            };

            println!("The value of y is {}", y);
            println!("The value of x is still {}", x);

            // You can return early from a function by using the return keyword and specifying a value,
            // but most functions return the last expression implicitly:
            let five = five();
            println!("The value of five is {}", five);
        }

        fn five() -> i32 {
            5
        }
    }

    fn control_flow() {
        let number = 3;

        if number != 0 {
            println!("Not zero");
        } else if number % 2 == 0 {
            println!("Even!");
        } else {
            println!("Odd!");
        }

        // While:

        // while index < 5 { ... }

        // For:

        // for element in a.iter() { ... }

        // Range:

        for number in (1..4).rev() {
            println!("{}!", number);
        }
        println!("LIFTOFF!!!");

        if_in_let_statement();
    }

    fn if_in_let_statement() {
        let condition = true;
        let number = if condition { 5 } else { 6 };

        println!("The value is {}", number);

        // let number = if condition { 5 } else { "six" };
        // Wouldn't compile.
        // Rust needs to know at compile time what type the number variable is.
    }
}
