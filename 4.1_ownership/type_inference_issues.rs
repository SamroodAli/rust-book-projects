fn main() {}

fn foo() {
    // here even though we know x will always be '1' as the if statement will always be true
    // but the compiler will assume that can be "hello";
    let x = if true { 1 } else { "hello" };
    assert_eq!(x + 1, 2); // so this won't work
}

// we never use s2 in this function
fn move_two(s1: String, s2: String) -> String {
    s1
}

fn main() {
    let (s1, s2) = (String::from("a"), String::from("b"));
    let s3 = move_two(s1, s2);

    // s2 is invalid even though s2 is actually not at all used in move_two
    println!("{} {}", s2, s3);
}
