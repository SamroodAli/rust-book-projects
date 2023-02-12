// There are both signed and unsigned integer types

fn main() {
    // unsigned integers
    let eight_bit: u8 = 100;
    println!("The value of eight_bit is {eight_bit}");

    let sixteen_bit: u16 = 10000;
    println!("The value of sixteen_bit is {sixteen_bit}");

    let thirty_two_bit: u32 = 1000000000;

    println!("The value of thirty_bit is {thirty_two_bit}");

    let sixty_four_bit: u64 = 10000000000000000000;

    println!("The value of sixty_four_bit {sixty_four_bit}");

    // signed integers
    let eight_bit: i8 = -100;
    println!("The value of eight_bit is {eight_bit}");

    let sixteen_bit: i16 = -10000;
    println!("The value of sixteen_bit is {sixteen_bit}");

    let thirty_two_bit: i32 = -1000000000;

    println!("The value of thirty_bit is {thirty_two_bit}");

    let sixty_four_bit: i64 = -1000000000000000000;
    println!("The value of sixty_four_bit {sixty_four_bit}");

    // based on architecture, either 32 bit or 64bit machine.

    let computer_architecture: usize = 1000000000000000000;

    // integer overflow ( DO NOT RELY ON THIS BEHAVIOR (bad practice));
    // eight bit can hold 0 - 255 numbers, 266 will wrap around to 0,
    // 267 will be 1, 268 will be 2 and so on

    // but this won't work in debug mode. in debug mode, you can will get a panic error.
    // in --release compilation, you get a 'wrapping around' behavior.
    let eight_bit_overflow: u8 = 256;
    let eight_bit_overflow_example_2: u8 = 257;
    let eight_bit_overflow_example_3: u8 = 257;

    // to be verified, this doesn't seem to compile
    // TODO: find if this actually works
    println!("The value of eight_bit_overflow for 256 {eight_bit_overflow}");
    println!("The value of eight_bit_overflow for 257 {eight_bit_overflow_example_2}");
    println!("The value of eight_bit_overflow for 258 {eight_bit_overflow_example_3}");
}
