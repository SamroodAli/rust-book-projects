fn main() {
    // we have slices other than string slices as well

    let array = [1, 2, 3, 4, 5];

    let slice = &array[..3];

    println!("{:?}", slice);
}
