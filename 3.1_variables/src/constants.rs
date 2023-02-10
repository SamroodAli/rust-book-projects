// https://nickymeuleman.netlify.app/garden/rust-let-const

// https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

// constants are immutable always. can't use the mut keyword
// you must declare the type of the variable.
// might not be stored in the heap since constants are known at compile time (to be verified)
// Constants can only be set to a constant expression, not to the result of a function call or anything that could only be determined at runtime.

const NUM: u8 = 10;

fn main() {
    println!("The value of num is {NUM}")
}
