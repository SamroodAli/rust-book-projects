#[derive(Debug)]
enum IppAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IppAddrKindWithData {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let four = IppAddrKind::V4;
    let six = IppAddrKind::V6;

    route(four);
    route(six);

    let other_four = IppAddrKindWithData::V4(198, 0, 0, 1);
    let other_six = IppAddrKindWithData::V6(String::from("::1"));

    println!("The other kind {:?} {:?}", other_four, other_six);
}

// we can use enums like any other type
fn route(ip_kind: IppAddrKind) {
    println!("Routing with {:?}", ip_kind);
}
