pub mod c06 {
    pub fn enums() {
        println!("*** Chapter 6 - Enums ***");

        // Enums and Pattern Matching
        defining_enums();

        // The Match Control Flow Operator
        match_control_flow_operator();

        // Running multiple lines of code in a match arm
        multiple_lines_match();

        // Patterns that Bind to Values
        pattern_matching();

        // Matching with Option<T>
        option_matching();

        // Matches are Exhaustive
        exhaustive_matches();

        // The _ Placeholder
        underscore_placeholder();

        // Concise Control Flow with the `if let` Syntax
        if_let();
    }

    pub fn defining_enums() {
        // Defining an Enum:
        enum IpAddrKind {
            V4,
            V6,
        }

        // Create instances like this:
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;

        // We can define a fn that takes any IpAddrKind:
        fn route(ip_kind: IpAddrKind) {}

        route(IpAddrKind::V4);
        route(IpAddrKind::V6);

        // To also store the actual IP address _data_, we can use a struct:
        struct IpAddr {
            kind: IpAddrKind,
            address: String,
        }

        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };

        // We can represent the same concept using just an enum. This new definition of the IpAddr enum says that both V4 and V6 variants will have associated String values:
        enum IpAddr2 {
            V4(String),
            V6(String),
        }

        let home = IpAddr2::V4(String::from("127.0.0.1"));
        let loopback = IpAddr2::V6(String::from("::1"));

        // If we wanted to store V4 addresses as four u8 values but still express V6 addresses as one String value, we wouldn’t be able to with a struct. Enums handle this case with ease:
        enum IpAddr3 {
            V4(u8, u8, u8, u8),
            V6(String),
        }

        let home = IpAddr3::V4(127, 0, 0, 1);
        let loopback = IpAddr3::V6(String::from("::1"));

        // Another example of an enum with a wide variaty of types embedded in its variants:
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        // Just as we’re able to define methods on structs using impl,
        // we’re also able to define methods on enums.
        // Here’s a method named call that we could define on our Message enum:

        impl Message {
            fn call(&self) {
                // method body would be defined here
            }
        }

        let m = Message::Write(String::from("hello"));
        m.call();

        // The body of the method would use self to get the value that we called the method on.
        // In this example, we’ve created a variable m that has the value
        // Message::Write(String::from("hello")), and that is what self will be in the body
        // of the call method when m.call() runs.
    }

    fn match_control_flow_operator() {
        // The power of match comes from the expressiveness of the patterns and
        // the fact that the compiler confirms that all possible cases are handled.

        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter,
        }

        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }

        println!("Value for penny: {}", value_in_cents(Coin::Penny));
        println!("Value for quarter: {}", value_in_cents(Coin::Quarter));

        // If a pattern matches the value, the code associated with that pattern is executed.
    }

    fn multiple_lines_match() {
        enum Salutation {
            Hello,
            Hi,
        }

        fn greet(greeting: Salutation) {
            // => { multi-line }
            // => single-line
            match greeting {
                Salutation::Hello => {
                    println!("Hello...");
                    println!(" World!");
                }
                Salutation::Hi => println!("Hi!"),
            }
        }

        greet(Salutation::Hello);
        greet(Salutation::Hi);
    }

    fn pattern_matching() {
        #[derive(Debug)] // so we can inspect the state in a minute
        enum UsState {
            Alabama,
            Alaska,
            // --snip--
        }

        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState), // <- Quarter variant also holds a UsState value
        }

        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}!", state);
                    25
                }
            }
        }

        println!(
            "Value for {:?}'s quarter: {}",
            UsState::Alabama,
            value_in_cents(Coin::Quarter(UsState::Alabama))
        );
    }

    fn option_matching() {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }

        let five = Some(5);

        let six = plus_one(five);
        println!("5 + 1 = {:?}", six); // Some(6)

        let none = plus_one(None);
        println!("None + 1 = {:?}", none); // None

        // You’ll see this pattern a lot:
        // -> match against an enum
        // -> bind a variable to the data inside
        // -> execute code based on it
    }

    fn exhaustive_matches() {
        // error[E0004]: non-exhaustive patterns: `None` not covered

        // ^ we didn’t cover every possible case!
        // Matches in Rust are exhaustive: we must exhaust every possibility
        // in order for the code to be valid.
    }

    fn underscore_placeholder() {
        // If we don't want to list out every possible value, we can the special pattern `_`:
        let some_u8_value = 0u8;
        match some_u8_value {
            1 => println!("one"),
            2 => println!("two"),
            _ => (), // wildcard pattern => unit value
                     // (we do nothing for all the other possible values)
        }

        fn match_me(x: &u8) -> &str {
            match x {
                1 => "one",
                2 => "two",
                3 => "three",
                _ => "anything else!",
            }
        }

        println!("Match 1: {:?}", match_me(&1));
        println!("Match 3: {:?}", match_me(&3));
        println!("Match 42: {:?}", match_me(&42));
    }

    fn if_let() {
        let some_u8_value = Some(3u8);
        // When we only care about a single value, we can use if let to match it:
        if let Some(3) = some_u8_value {
            println!("if let three");
        }

        // Choosing between match and if let depends on what you’re doing in
        // your particular situation and whether gaining conciseness is
        // an appropriate trade-off for losing exhaustive checking.

        // if let ==  syntax sugar for a match that runs code when the value
        //            matches one pattern and then ignores all other values.

        // if let + else
        #[derive(Debug)]
        enum UsState {
            Alaska,
            // ...
        }
        #[derive(Debug)]
        enum Coin {
            Penny,
            // ...
            Quarter(UsState),
        }
        let coin = Coin::Quarter(UsState::Alaska);
        let mut count = 0;
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}!", state);
            count += 1;
        } else {
            println!("I don't know coin {:?}!", coin);
        }
        println!("count: {}", count);
    }
}
