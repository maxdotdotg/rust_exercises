// enums are good for hard limits on types,
// in this case ip address kinds
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddrKind_with_types {
    V4(String),
    V6(String),
}

// using the enum to define a struct
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let v_four = IpAddrKind::V4;
    let v_six = IpAddrKind::V6;

    // instantiate the struct
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // alternately, we can do it differently with a tighter enum definition
    let home_again = IpAddrKind_with_types::V4(String::from("127.0.0.1"));
    let loop_again = IpAddrKind_with_types::V6(String::from("::1"));

    println!("{:?}", home_again)
}

fn route(ip_kind: IpAddrKind) { }
