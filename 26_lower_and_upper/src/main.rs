// Topic: Browsing standard library documentation
//
// Requirements:
// * Print a string in lowercase and uppercase
//
// Notes:
// * Utilize standard library functionality to
//   transform the string to lowercase and uppercase
// * Use 'rustup doc' in a terminal to open the standard library docs
// * Navigate to the API documentation section
// * Search for functionality to transform a string (or str)
//   to uppercase and lowercase
//   * Try searching for: to_uppercase, to_lowercase

fn uppercase_con(i: &str){
    println!("main was: {} , uppercase is: {}",i,i.to_uppercase());
}

fn lowercase_con(i: &str){
    println!("main was: {} , lowercase is: {}",i,i.to_lowercase());
}

fn main() {
    let low = "abc".to_owned();
    let upp = "XYZ".to_owned();
    uppercase_con(&low);
    lowercase_con(&upp);
}
// exercise done