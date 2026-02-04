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

struct MinAge {
    age : u8,
}

fn age_checker(age: MinAge) -> Result<() , String> {
    if age.age > 20 {
        Ok(())
    }else {
        Err("Oops! not possible for this age".to_owned())
    }
}

fn main() {
    let customer_age = MinAge { age : 21};
    match age_checker(customer_age) {
        Ok(_) => println!("Come in Welcome!"),
        Err(message) => println!("{:?}",message),
    }
}

