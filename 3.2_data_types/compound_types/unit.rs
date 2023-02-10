// The tuple without any values has a special name, unit.
// This value and its corresponding type are both written ()
// and represent an empty value or an empty return type.
// Expressions implicitly return the unit value if they don’t return any other value.

fn main() {
    let empty = ();

    println!("empty value {:?}", empty);
}
