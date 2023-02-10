fn f(x: i32) -> i32 {
    x + 1
}
// In rust, you can use a block as an express. The last expression will be
//  what the block will be evaluated as

fn main() {
    println!(
        "{}",
        f(
            // this block here evaluates to 2 from the code below
            {
                let y = 1;
                // last line, WITHOUT SEMI COLON is the the value
                y + 1
            }
        )
    );
}
