use std::collections::HashMap;

#[derive(Debug)]
enum Status {
    Active,
    Inactive,
    Banned,
}

#[derive(Debug)]
struct User {
    name: String,
    age: u8,
    status: Status,
}

fn main() {
    let mut users: HashMap<u32, User> = HashMap::new();

    users.insert(
        1,
        User {
            name: String::from("Ali"),
            age: 25,
            status: Status::Active,
        },
    );

    users.insert(
        2,
        User {
            name: String::from("Sara"),
            age: 30,
            status: Status::Banned,
        },
    );

    for (id, user) in &users {
        println!("User ID: {}", id);

        match user.status {
            Status::Active => println!("{} is active", user.name),
            Status::Inactive => println!("{} is inactive", user.name),
            Status::Banned => println!("{} is banned", user.name),
        }
    }
}
