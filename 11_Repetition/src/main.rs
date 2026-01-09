fn main() {
    println!("Hello, world!");
    println!("2026.01.09");
    println!("our contry internet is down by gov");
    loopex();
    whileex();
    matchexe();
}

fn loopex() {
    println!("========== Loop ==========");
    let mut a = 5;
    loop {
        println!("loop remaining : {:?}" , a);
        a = a -1 ;
        if a == -1 {
            break;
        }
    }
}

fn whileex() {
    println!("========== While ==========");
    let mut a = 5 ;
    while a >= 0 {
        println!("while remaining : {:?}" , a);
        a = a - 1 ;
    }
}

fn matchexe() {
    println!("========== Match ==========");
    let a = 6 ; 
    match a {
        3 => println!("this is 1 * 3 "),
        6 => println!("this is 2 * 3 "),
        9 => println!("this is 3 * 3 "),
        _ => println!("other numbers"),
    }
}