enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let anotherfour = IpAddrKind::V4;
    println!("{}", String::from("hi there"));
}

fn route(ip_type: IpAddrKind) {}
