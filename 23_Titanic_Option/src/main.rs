struct Locker {
    name: String ,
    assignment: Option<i32>
}

fn print_info(a: Locker) {
    println!("");
    println!("***********");
    println!("the name of locker owner : {}",a.name);
    match a.assignment {
        Some(number) => println!("number: {}",number),
        None => println!("there is no locker for class C passengers"),
    }
    
}

fn main() {
    let s1 = Locker{
        name: "Jack".to_owned(),
        assignment: None,
    };
    let s2 = Locker{
        name: "Rose".to_owned(),
        assignment: Some(23),
    };
    let s3 = Locker{
        name: "Captain".to_owned(),
        assignment: Some(1),
    };
    
    print_info(s1);
    print_info(s2);
    print_info(s3);
}



