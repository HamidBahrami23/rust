use std::collections::HashMap;

fn check_age(name: &str, users: &HashMap<String, u8>) -> Result<u8, String> {
    let age = users.get(name).ok_or("User not found")?;

    if *age < 18 {
        Err("Age is below 18".to_string())
    } else {
        Ok(*age)
    }
}

fn main() {
    let mut users: HashMap<String, u8> = HashMap::new();

    users.insert("Alice".to_string(), 22);
    users.insert("Bob".to_string(), 16);

    match check_age("Alice", &users) {
        Ok(age) => println!("Alice is allowed, age: {}", age),
        Err(err) => println!("Error: {}", err),
    }

    match check_age("Bob", &users) {
        Ok(age) => println!("Bob is allowed, age: {}", age),
        Err(err) => println!("Error: {}", err),
    }

    match check_age("Charlie", &users) {
        Ok(age) => println!("Charlie is allowed, age: {}", age),
        Err(err) => println!("Error: {}", err),
    }
}
//just for filling my day in 2026.02.11
//today was a bad day for me
//but I ried to learn more in git
//so maybe u can't see any progress
//gl next days
