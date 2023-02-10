fn main() {
    // with data types in the heap, we move values instead of copying them

    let s1 = String::from("string a ");
    // here the value of s1 is passed to s2 and s1 is no longer valid.
    let s2 = s1;

    println!("{s}")
}
