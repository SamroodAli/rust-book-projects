struct User {
    name: String,
    age: u8,
    email: String,
    is_active: bool,
}

fn main() {
    let user = build_user(
        String::from("Samrood"),
        26,
        String::from("samrood's email"),
        true,
    );

    println!(
        "My name is {}. I am {} years old. My email address is {}. I am {} in social media.",
        user.name,
        user.age,
        user.email,
        match user.is_active {
            true => "active",
            false => "not active",
        },
    );
}

fn build_user(name: String, age: u8, email: String, is_active: bool) -> User {
    User {
        name,
        age,
        email,
        is_active,
    }
}
