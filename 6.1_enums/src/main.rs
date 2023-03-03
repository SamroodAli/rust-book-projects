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

#[derive(Debug)]
enum Message {
    Quit,
    Write(String),
    ChangeColor(i32, i32, i32),
    Move { x: u32, y: u32 },
}

fn main() {
    let four = IppAddrKind::V4;
    let six = IppAddrKind::V6;

    route(four);
    route(six);

    let other_four = IppAddrKindWithData::V4(198, 0, 0, 1);
    let other_six = IppAddrKindWithData::V6(String::from("::1"));

    println!("The other kind {:?} {:?}", other_four, other_six);

    let next_move: Message = Message::Quit;
    let next_move: Message = Message::ChangeColor(200, 200, 200);
    let next_move: Message = Message::Write(String::from("Some message"));
    let next_move: Message = Message::Move { x: 200, y: 200 };
}

// we can use enums like any other type
fn route(ip_kind: IppAddrKind) {
    println!("Routing with {:?}", ip_kind);
}
