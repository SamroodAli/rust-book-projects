fn main() {
    let name = String::from("Samrood");
    // since the value of the owner name is heap allocated, the value of name is
    // 'moved' to foo
    foo(name);

    // println!("{}", name); // can' access name, because it has already
    //  been moved

    let moving_friend = String::from("Abhiram");

    // moving_friend is moved, the owner (moving_friend) becomes invalid.
    let (new_friend, returning_friend) = move_back(moving_friend);

    // println!("{}", moving_friend); // won't work as moving_friend is invalid after the move

    println!("{} {}", new_friend, returning_friend);
}

fn foo(s: String) {
    // s gets the value of String
    println!("The value of s is {}", s);
    // since we are not returning s here, s is 'dropped'
}

// we can also 'move' back values ( both new ones and arguments)
fn move_back(incoming_friend: String) -> (String, String) {
    let new_friend = String::from("Shamlid");

    return (new_friend, incoming_friend);
}
