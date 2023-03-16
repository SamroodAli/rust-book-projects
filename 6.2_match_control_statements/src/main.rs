fn main() {
    match_with_enums();
    match_with_options();
    match_with_catch_all();
}

// match with enums

#[derive(Debug)]
enum UsStates {
    Alabama,
    Alaska,
}

enum Coin {
    Nickel,
    Penny,
    Dime,
    Quarter(UsStates),
}

fn match_with_enums() {
    let coin = Coin::Nickel;
    println!("{}", value_in_cents(coin));

    let coin = Coin::Dime;
    println!("{}", value_in_cents(coin));

    let coin = Coin::Penny;
    println!("{}", value_in_cents(coin));

    let coin = Coin::Quarter(UsStates::Alabama);
    println!("{}", value_in_cents(coin));

    let coin = Coin::Quarter(UsStates::Alaska);
    println!("{}", value_in_cents(coin));
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("We got a coin from {:?}", state);
            25
        }
    }
}

// match with options

fn match_with_options() {
    let num = Some(10);
    plus_one(num);
}

fn plus_one(num: Option<i32>) -> Option<i32> {
    match num {
        Some(num) => {
            let answer = Some(num + 1);
            println!(
                "Adding 1 to {num} gives you {:?}",
                match answer {
                    Some(n) => n,
                    None => num + 1,
                }
            );
            answer
        }
        None => None,
    }
}

// match with catch all patterns

#[derive(Debug)]
enum Roll {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

fn match_with_catch_all() {
    let roll: Roll = Roll::Five;

    match roll {
        Roll::One => {
            println!("Rolled a one");
        }
        Roll::Six => {
            println!("Rolled a six")
        }
        other => {
            println!("Rolled {:?}, roll again. Need a 'one' or a 'six'", other)
        }
    }
}
