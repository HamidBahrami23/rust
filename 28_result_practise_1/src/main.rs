fn paswwoed_checker(pass: &str) -> Result<passdata , String> {
    if pass == "hold the door" {
        Ok(passdata::new("hodor")),
    } else {
        Err("your not the one".to_owned())
    }
}

fn main() {
    let input_pass = "007";
    match paswwoed_checker(input_pass) {
        Ok(_) => println!("congrates hero"),
        Err(text) => println!("{:?}", text),
    }
}
