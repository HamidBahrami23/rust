/*

## 🧠 1. Simple Console Application with Different Functions

### **Project Goal**
Create a basic console application that demonstrates various functions (addition, subtraction, multiplication, division) while utilizing variables, data types, and control flow mechanisms (if/else, match).

### **What to Implement**
- Create multiple functions for basic arithmetic operations
- Use different data types (integers, floating-point numbers)
- Implement control flow structures (if/else statements)
- Use match expressions for operation selection
- Handle user input from console
- Display results in formatted output
- Implement error handling for invalid operations (like division by zero)
- Use variables with different scopes and lifetimes

### **What NOT to Implement**
- Complex mathematical calculations beyond basic arithmetic
- GUI interfaces or graphical elements
- External database connections
- Advanced memory management features
- Multi-threading or async operations
- Complex data structures beyond basic types

### **Tools and Technologies**
- Rust compiler (rustc)
- Standard library only (no external crates needed)
- Basic console input/output operations
- Variable declaration and assignment
- Basic control flow statements

*/
use std::io;

fn read_number() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");

    input.trim().parse::<i32>().expect("Please enter a number")
}

/*
this is ai suggeststion that I still can't unerstand it
fn read_number() -> i32 {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<i32>() {
            Ok(n) => return n,
            Err(_) => println!("Please enter a valid number:"),
        }
    }
}

*/


fn sumf(a: i32 , b: i32) -> i32{
    a + b
}

fn subf(a: i32 , b: i32) -> i32{
    a - b
}

fn mulf(a: i32 , b: i32) -> i32 {
    a * b
}

fn divf(a: i32 , b: i32) -> Option<f64> {
    if b == 0 {
        None
    } else {
        Some(a as f64 / b as f64)
    }
}


enum Console {
    Sum ,
    Sub ,
    Mul ,
    Div ,
}

fn main() {
    println!("Welcome to my first Project in Rust");
    
    println!("Enter first number:");
    let num1 = read_number();

    println!("Enter second number:");
    let num2 = read_number();

    println!("our numbers are :");
    println!("number 1 : {:?}", num1);
    println!("number 2 : {:?}", num2);
    println!("Now let's make a tiny magic:");
    println!("");

    let operations = vec![
        Console::Sum ,
        Console::Sub ,
        Console::Mul ,
        Console::Div ] ;

    for i in operations {

        match i {
            Console::Sum => println!("Sum = {}",sumf(num1, num2)),
            Console::Sub => println!("Sub = {}",subf(num1, num2)),
            Console::Mul => println!("Mul = {}",mulf(num1, num2)),
            Console::Div => match divf(num1, num2) {
                Some(result) => println!("Div = {}", result),
                None => println!("Div = Cannot divide by zero"),
            }
        }

    }

}
