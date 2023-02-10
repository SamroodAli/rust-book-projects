// rust doesn't care where we declare functions as long as it's in the scope of the caller.
fn foo() {
    println!("This is from function foo");
}

fn main() {
    println!("This is from the main function");
    another_function();
    foo();

    can_take_arguments(20);

    can_take_multiple_arguments(10, true);
}

fn another_function() {
    println!("This is from another function ");
}

fn can_take_arguments(num: i32) {
    println!("The value of num is {num}");
}

fn can_take_multiple_arguments(num: i32, is_awesome: bool) {
    println!("The arguments are {num} {is_awesome}");
}
