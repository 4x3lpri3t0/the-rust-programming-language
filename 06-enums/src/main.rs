// Enums and Pattern Matching

fn main() {
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

    // TODO ...
}
