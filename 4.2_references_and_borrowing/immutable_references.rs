fn main() {
    let s1 = String::from("s1");

    let len = calculate_length(&s1);

    println!("{len} {s1}");
}

// here we are taking an immutable reference, and not the ownership of String
// this is called borrowing.
fn calculate_length(s: &String) -> usize {
    s.len()
}

// what if we take an immutable reference and try to modify it

fn add_to_string(s: &String) {
    s.push_str("modified"); // won't compile! as we had a immutable reference,
                            // we are not allowed to change/modify the value // Sweet!
}
