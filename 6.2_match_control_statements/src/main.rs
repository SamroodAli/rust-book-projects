enum Coin {
    Nickel,
    Penny,
    Dime,
    Quarter,
}

fn main() {
    let coin = Coin::Nickel;
    println!("{}", value_in_cents(coin));

    let coin = Coin::Dime;
    println!("{}", value_in_cents(coin));

    let coin = Coin::Penny;
    println!("{}", value_in_cents(coin));

    let coin = Coin::Quarter;
    println!("{}", value_in_cents(coin));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
