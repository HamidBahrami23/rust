fn main() {
    let num1: i32 = 20;
    let num2: i32 = 27;

    // Call the function and store the result
    let result = add_numbers(num1, num2);

    // Print the result to the console
    println!("The sum of {0} and {1} is: {2}", num1, num2, result);
}

// Function that takes two integers and returns their sum
fn add_numbers(a: i32, b: i32) -> i32 {
    // In Rust, the last expression without a semicolon is returned
    a + b
}