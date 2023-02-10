fn main() {
    // Note that we specify char literals with single quotes,
    //  as opposed to string literals, which use double quotes.
    //  Rust’s char type is four bytes in size and represents
    // a Unicode Scalar Value, which means it can represent a lot
    // more than just ASCII.
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
