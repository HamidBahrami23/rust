enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let n = 3;
    match n {
        3 => println!("three"),
        other => println!("number: {:?}", other), // other is variable that defined
    }

    let flat = Discount::Flat(2);
    match flat {
        Discount::Flat(2) => println!("flat 2"), // we can also use Discount::Flat(_) for ignoring number inside
        Discount::Flat(amount) => println!("flat discount of {:?}", amount),
        _ => (), // () means return nothing
    }

    let concert = Ticket {
        event: "Concert".to_owned(),
        price: 49,
    };
    match concert {
        Ticket {price: 32 , event} => println!("event @ 49$ = {:?}", event),
        Ticket {price , ..} => println!("Price = {:?}", price), // .. means ignore other fields
    }
}
