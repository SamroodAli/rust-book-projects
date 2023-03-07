fn main() {
    let value: Option<&str> = Some("value is available");

    match value {
        Some(string) => {
            println!("{string}");
        }
        None => {
            println!("There is no value");
        }
    }
}
