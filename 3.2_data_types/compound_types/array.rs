fn main() {
    // arrays are fixed size data types, holding multiple values of the same type
    // they are allocated in the stack, and not the heap.
    // useful when we know the size of the array
    // accessed with array[index]

    let array = [1, 2, 3, 4, 5];

    println!("The array is {:?}", array);

    // type annotation
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    println!("The array is {:?}", array);

    println!("the first value is {:?}", array[0]);
}
