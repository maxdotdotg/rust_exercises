// enums are good for hard limits on types,
// in this case ip address kinds
enum IpAddrKind {
    V4(String),
    V6(String)
}

// using the enum to define a struct
struct IpAddr {
    kind: IpAddrKind,
    address: String.
}

fn main() {
    let v_four = IpAddrKind::V4;
    let v_six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    }
}

fn route(ip_kind: IpAddrKind) { }
