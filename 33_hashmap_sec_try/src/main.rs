use std::collections::HashMap;

struct Contents {
    content: String
}
fn main() {
    let mut lockers = HashMap::new();
    lockers.insert(1, Contents { content: "Shirts".to_owned()});
    lockers.insert(2, Contents { content: "Books".to_owned()});
    lockers.insert(3, Contents { content: "stuff".to_owned()});

    for (number , item) in lockers.iter() {
        println!("number: {:?} , Content: {:?}", number , item.content);
    }
}
