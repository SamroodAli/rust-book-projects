fn main() {
    let reference_to_nothing = dangle(); // this won't compile as
                                         // this is a dangling reference
}

fn dangle() -> &String {
    let owner = String::from("hello");

    // here we are returning an immutable reference
    // but the owner leaves scope here and hence this value will be dropped
    // and the reference will be dangling but rust does not compile this.
    return &s;
    // to fix this, we can move ownership instead by returning the String
}
