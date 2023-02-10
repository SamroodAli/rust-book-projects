fn main() {
    // A tuple is a general way of grouping together a number of values with a
    // variety of types into one compound type.
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.

    let name = "Samrood";
    let tuple: (&str, i32, bool) = (name, 26, true);

    println!("{} {} {}", tuple.0, tuple.1, tuple.2);

    // println!("{} {} {}", tuple.0, tuple.1, tuple[2]); //won't work

    // destructing exists
    let (my_name, my_age, is_awesome) = tuple;

    println!("{my_name} {my_age} {is_awesome}");
}
