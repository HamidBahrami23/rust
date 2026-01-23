// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum TicketKind {
    Backstage(String),
    Vip(String),
    Standard,
}

struct TicketInfo {
    price: f64,
    ticket_kind: TicketKind,
}

fn print_info(a: &TicketInfo) {

    match &a.ticket_kind {
        TicketKind::Backstage(name) => println!("**Backstage for: {}",name),
        TicketKind::Vip(name) => println!("**VIP for: {}", name),
        TicketKind::Standard => println!("**Standard"),
    }
    println!("Price: {}" , a.price);
}

fn main() {
    let list_tickets = vec![
        TicketInfo { 
            price : 49.9 ,
            ticket_kind : TicketKind::Standard
            },
        TicketInfo {
            price: 79.9 ,
            ticket_kind: TicketKind::Backstage("John".to_owned()),
        },
        TicketInfo {
            price: 99.9 ,
            ticket_kind: TicketKind::Vip("Kate".to_owned()),
        },
        TicketInfo {
            price: 99.9 ,
            ticket_kind: TicketKind::Vip("Luke".to_owned()),
        }
    ];
    let mut num = 1;
    for i in &list_tickets {
        println!("number: {}" , num );
        print_info(i);
        println!("");
        num = num + 1;
    }
}



