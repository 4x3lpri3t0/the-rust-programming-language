pub mod c06 {
    pub fn enums() {
        println!("*** 6 - ENUMS ***");

        // Enums and Pattern Matching

        // Defining an Enum
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

        // Just as we’re able to define methods on structs using impl, we’re also able to define methods on enums. Here’s a method named call that we could define on our Message enum:

        impl Message {
            fn call(&self) {
                // method body would be defined here
            }
        }

        let m = Message::Write(String::from("hello"));
        m.call();

        // The body of the method would use self to get the value that we called the method on. In this example, we’ve created a variable m that has the value Message::Write(String::from("hello")), and that is what self will be in the body of the call method when m.call() runs.

        // The Option Enum and Its Advantages Over Null Values

        // TODO ...
    }
}
