
struct age {
    age: u8,
}



fn main() {
    let mut names = hashmap::new();
    names.insert("Sara", age {age : 34 } );



    match i in names("Sara") {
        Some(num) => println!("the age of sara is {:?}",num.age ),
        None => println!("user not found!")
    }
}
