#[derive(Debug)]
enum IppAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IppAddrKind::V4;
    let six = IppAddrKind::V6;

    route(four);
    route(six);
}

// we can use enums like any other type
fn route(ip_kind: IppAddrKind) {
    println!("Routing with {:?}", ip_kind);
}
