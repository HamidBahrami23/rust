use std::collections::HashMap;

#[derive(Debug)]
enum LangLevel {
    A1,
    A2,
    B1,
    B2,
    C1,
    C2,
}
struct Age {
    age: u8,
    weight: f32,
    language_level:LangLevel,
}



fn main() {
    let mut names = HashMap::new();
    names.insert("Sara", Age{ age: 34 ,  weight:53.0, language_level: LangLevel::A2,} );
    names.insert("Linus", Age {age: 47 , weight: 76.2 , language_level: LangLevel::C2 ,});

    match names.get("Linus") {
        Some(num) => println!("the age of Linus is {:?} , and the weight is : {:?} , and also his Language LVL is : {:?}",num.age , num.weight , num.language_level ),
        None => println!("user not found!")
    }
}
