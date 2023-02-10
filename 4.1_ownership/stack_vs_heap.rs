fn main() {
    // some data types are in the stack.
    // others are in the heap.
    // both the stack and the heap are parts of
    // memory available to your code to use at runtime.
    // all data stored on the stack must have fixed size.
    // data with an unknown size at compile tme or size that might change must be
    // stored on the heap instead.
    // the heap is more expensive.

    // ownership rules
    // 1. Each value in rust has an owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, value will be dropped.

    {
        // each value in rust has an owner
        let name = "Samrood";
        // there can only be one owner at a time.
        // if we assign the value of a variable to another variable.
        // For stack data types, they are copied over. two owners, two values.
        // for heap, the ownership is moved. the previous owner is invalid,
        // the new owner points to the value
        let name2 = name;

        println!("{name} , {name2}");
        // the values in name and name2 are dropped here since their scope is over.
    }
}
