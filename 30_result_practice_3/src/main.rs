// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
    age: u8,
}

fn can_make_restricted_purchase(customer: &Customer) -> Result<(), String> {
    if customer.age >= 21 {
        Ok(())
    } else {
        Err(format!("Customer is too young: age {}", customer.age))
    }
}

fn main() {
    let customer = Customer { age: 19 };

    match can_make_restricted_purchase(&customer) {
        Ok(()) => println!("Purchase approved"),
        Err(reason) => println!("Purchase denied: {}", reason),
    }
}

