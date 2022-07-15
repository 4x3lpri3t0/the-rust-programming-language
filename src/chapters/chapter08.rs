pub mod c08 {
    pub fn common_collections() {
        println!("*** Chapter 8 - Common Collections ***");

        // Storing Lists of Values with Vectors
        vectors();

        // Storing UTF-8 Encoded Text with Strings
        strings();

        // Storing Keys with Associated Values in Hash Maps
        hash_maps();
    }

    fn vectors() {
        // Creating a new Vector:
        let v: Vec<i32> = Vec::new();
        // vec! macro for convenience:
        let v = vec![1, 2, 3]; // -> infers that the type is i32

        // Updating a Vector:
        let mut v = Vec::new();

        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);

        // Dropping a Vector Drops Its Elements:
        {
            let v = vec![1, 2, 3, 4];
            // do stuff with v
        } // <- goes out of scope and its freed here

        // Reading Elements of Vectors:
        {
            let v = vec![1, 2, 3, 4, 5];
            let third: &i32 = &v[2];
            println!("The third element is {}", third);

            match v.get(2) {
                Some(third) => println!("The third element is {}", third),
                None => println!("There is no third element."),
            }

            // let does_not_exist = &v[100];
            // ^ panicks at compilation time.
            // -> better when you want to crash if there’s an attempt to access an element past the end of the vector.

            let does_not_exist = v.get(100);
            // ^ returns index without panicking.
        }

        // Iterating over the Values in a Vector:
        {
            let v = vec![100, 32, 57];
            for val in &v {
                println!("{}", val);
            }
        }

        // Iterating over mutable refs to each element in a mutable vector in order to make changes to the elements:
        {
            let mut v = vec![100, 32, 57];
            for val in &mut v {
                *val += 50;
            }
            println!("{:?}", v);
        }

        // Using an Enum to Store Multiple Types:
        // When we need to store elements of a different type in a vector, we can define and use an enum!
        // We can define an enum whose variants will hold the different value types, and then all the enum variants will be considered the same type: that of the enum.
        #[derive(Debug)]
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(42),
            SpreadsheetCell::Text(String::from(
                "Can use vectors that contain diff types thanks to enums!",
            )),
            SpreadsheetCell::Float(33.01),
        ];
        // ^ Defining an enum to store values of different types in one vector.
        println!("Vec + enums ftw! {:?}", row);
        // Using an enum plus a match expression means that Rust will ensure at compile time that every possible case is handled
    }

    fn strings() {
        // New Rustaceans commonly get stuck on strings for a combination of three reasons:
        // * Rust’s propensity for exposing possible errors
        // * Strings being a more complicated data structure than many programmers give them credit for
        // * UTF-8
        // ^ These factors combine in a way that can seem difficult when you’re coming from other programming languages.

        // Strings are implemented as a collection of bytes + plus some methods to provide useful functionality when those bytes are interpreted as text.

        // What Is a String?
        // Rust has only one string type in the core language, which is the string slice str that is usually seen in its borrowed form &str
        // Slices: references to some UTF-8 encoded string data stored elsewhere.

        // When Rustaceans refer to “strings” in Rust, they usually mean the String and the string slice &str types, not just one of those types. Both types are used heavily in Rust’s standard library, and both String and string slices are UTF-8 encoded.

        // Creating a New String:
        let mut s = String::new();

        {
            let data = "initial contents";
            println!("{}", data);

            let s = data.to_string();
            println!("{}", s);

            // to_string() method also works on a literal directly:
            let s = "initial contents".to_string();
            println!("{}", s);
            // ^ Using the String::from function to create a String from a string literal
        }

        // Updating a String:

        {
            // Appending with push and push_str
            let mut s = String::from("foo");
            s.push_str("bar");
            println!("{}", s);
        }

        {
            // Adding one char
            let mut s = String::from("lo");
            s.push('l');
            println!("{}", s);
        }

        {
            // Concatenation with the + Operator or the format! Macro
            let s1 = String::from("Hello, ");
            let s2 = String::from("world!");
            let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

            /*
            The reason s1 is no longer valid after the addition and the reason we used a
            reference to s2 has to do with the signature of the method that gets called
            when we use the + operator.
            The + operator uses the add method, whose signature looks something like this:
            fn add(self, s: &str) -> String {
            */

            // The reason we’re able to use &s2 in the call to add is that the compiler can coerce the &String argument into a &str .
            // When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..] .

            // Because add does not take ownership of the s parameter, s2 will still be a valid String after this operation.Second, we can see in the signature that add takes ownership of self, because self does not have an & .
        }

        {
            // format! macro (for more complicated concatenating):__rust_force_expr!
            let s1 = String::from("tic");
            let s2 = String::from("tac");
            let s3 = String::from("toe");
            let s = format!("{}-{}-{}", s1, s2, s3);
            println!("{}", s);
        }

        // Methods for Iterating over Strings
        {
            // .chars() method:
            for c in "Здравствуйте".chars() {
                println!("{}", c);
            }
        }
    }

    fn hash_maps() {
        use std::collections::HashMap;
        // HashMap<K, V> stores a mapping of keys of type K to values of type V.

        // Creating a New Hash Map:
        {
            let mut scores = HashMap::new();

            scores.insert(String::from("Blue"), 10);
            scores.insert(String::from("Yellow"), 90);
            scores.insert(String::from("Blue"), 11); // Overwrite

            // Insert if key does not have a value yet:
            scores.entry(String::from("Red")).or_insert(120);

            println!("{:?}", scores);

            // Accessing values:
            let team_name = String::from("Blue");
            let score = scores.get(&team_name);

            // Iterate:
            for (k, v) in scores {
                println!("{}: {}", k, v);
            }

            // Overwriting after borrowing values by iteration fails:
            // scores.insert(String::from("Blue"), 11); // Fails!
        }

        // Update val based on old one:
        {
            let text = "hello world wonderful world";
            let mut map = HashMap::new();

            for word in text.split_whitespace() {
                let count = map.entry(word).or_insert(0);
                *count += 1
            }

            println!("{:?}", map);
        }
    }
}
