// this code give us error cuz ownership - Move in Rust
// comment code below and use last one to solution using Borrow
// /*
enum Light {
    Bright,
    Dull,
}

fn display_light(light: Light){
    match light {
        Light::Bright => println!("bright"),
        Light::Dull => println!("dull"),
    }
}


fn main() {
    let dull = Light::Dull;
    display_light(dull);
    display_light(dull);
}
// */



/* //////////////////////////////////////////
result in Terminal:
error[E0382]: use of moved value: `dull`
  --> src/main.rs:17:19
   |
15 |     let dull = Light::Dull;
   |         ---- move occurs because `dull` has type `Light`, which does not implement the `Copy` trait
16 |     display_light(dull);
   |                   ---- value moved here
17 |     display_light(dull);
   |                   ^^^^ value used here after move
   |
note: consider changing this parameter type in function `display_light` to borrow instead if owning the value isn't necessary
  --> src/main.rs:6:25
   |
 6 | fn display_light(light: Light){
   |    -------------        ^^^^^ this parameter takes ownership of the value
   |    |
   |    in this function
note: if `Light` implemented `Clone`, you could clone the value
  --> src/main.rs:1:1
   |
 1 | enum Light {
   | ^^^^^^^^^^ consider implementing `Clone` for this type
...
16 |     display_light(dull);
   |                   ---- you could clone this value

For more information about this error, try `rustc --explain E0382`.
error: could not compile `main` (bin "main") due to 1 previous error

*//////////////////////////////////////////

// solution: using Borrow
// uncomment code below: 

/*

enum Light {
    Bright,
    Dull,
}

fn display_light(light: &Light){
    match light {
        Light::Bright => println!("bright"),
        Light::Dull => println!("dull"),
    }
}


fn main() {
    let dull = Light::Dull;
    display_light(&dull);
    display_light(&dull);
}

*/
// today was realy bad day for me (2026.01.13)
// I lost my pet.