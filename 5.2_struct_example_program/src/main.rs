fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rectangle)
    );

    println!("printing the rectangle {:#?}", &rectangle);

    print_with_dbg();
}

// METHOD 1: Without structs
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// METHOD 2: With tuple structs
// struct Rectangle(u32, u32);
// fn area(rectangle: Rectangle) -> u32 {
//     rectangle.0 * rectangle.1
// }

// METHOD 3: With structs

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

fn print_with_dbg() -> Rectangle {
    let scale = 2;

    let rectangle = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(rectangle)
}
