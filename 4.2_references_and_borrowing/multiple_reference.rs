fn main() {
    let mut s1 = String::from("string s1");

    let r1 = &s1;
    let r2 = &s1; // we have more than one immutable references

    println!("{r1} {r2}"); // works

    let r3 = &mut s1; // we can can have one mutable reference

    println!("{r3}"); // this works

    let r5 = &mut s1;

    println!("{r5}"); // also works

    // we cannot use two mutable references concurrently or at the same time
    // println!("{r5} {r3}"); // but this doesn't work.

    // we also can't have a mutable ref nd immutable ref together
    // println!("{r1} {r3}");

    // basically, only multiple immutable references are allowed.
    // mutable and any other are not allowed
    // we can use them, just not at the same time.
    // Rust uses NLL (Non lexical lifetimes) to assess the scope of a reference.
    // scope of a reference is from creation to last usage.
}
