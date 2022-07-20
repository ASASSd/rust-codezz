struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        active: user1.active,
        username: user1.username.clone(),
        email: user1.email,
        sign_in_count: user1.sign_in_count,
    };

    println!("User 2 username is {}", user2.username);
    println!("User 1 username is {}", user1.username);
}
