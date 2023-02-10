fn main() {
    let s = String::from("Hello world");

    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];

    println!("{hello} {world}");

    // if we want to start from zero, we can skip the initial value
    let hello: &str = &s[..5];

    // if want to go till the end, we can the skip the second value.
    let world: &str = &s[6..];

    println!("{hello} {world}");

    let hello_world: &str = &s[..];

    println!("{hello_world}");
}
