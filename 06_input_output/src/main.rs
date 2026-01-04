use std::io; // Import input/output library

fn main() {
    // Prompt the user for their name
    println!("Please enter your name:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    // Prompt the user for their age
    println!("Please enter your age:");
    let mut age_input = String::new();
    io::stdin()
        .read_line(&mut age_input)
        .expect("Failed to read line");

    // Convert string input to an integer
    let age: u32 = age_input.trim().parse().expect("Please type a number!");

    // Call the function to calculate age in days
    let age_in_days = calculate_days(age);

    // Print the final result
    println!("Hello, {}You are approximately {} days old!", name, age_in_days);
}

// Function that takes a u32 and returns a u32
fn calculate_days(years: u32) -> u32 {
    let days_in_year = 365;
    years * days_in_year // Return the calculation
}