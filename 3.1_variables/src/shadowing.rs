fn main() {
    let x = 10;
    println!("The value of x initially is {x}");

    {
        let x = 20;
        println!("the value of x in inner scope is {x}, inner shadowing");
    }

    println!("The value of x is {x}, has block scoping");

    let x = 30;

    println!("The value of x is {x}");

    let x = "Samrood";
    println!("The value of x is {x}, doesn't have to be the same type");
}
