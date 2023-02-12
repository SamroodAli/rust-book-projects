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

    let user2 = User {
        // the name here takes priority
        name: String::from("Sanjeed"),
        age: 18,
        email: String::from("sanjeed's email"),
        // base class, even though it's last, only remaining fields are added.
        ..user
    };

    println!(
        "My name is {}. I am {} years old. My email address is {}. I am {} in social media.",
        user2.name,
        user2.age,
        user2.email,
        match user.is_active {
            true => "active",
            false => "not active",
        },
    );

    // we can only access user because we never moved heap values (String in name and email)
    // in user2. if we had not given name or email in user2, the values from
    // user would have moved to user2 and user would not be valid anymore.
    // stack stored values like age, is_active are copied.
    println!("{}", user.email); // this works because we set name and email.
}

fn build_user(name: String, age: u8, email: String, is_active: bool) -> User {
    User {
        name,
        age,
        email,
        is_active,
    }
}
