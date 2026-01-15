// Topic: Implementing functionality with the impl keyword (Exercise)
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics


enum Color {
    Red,
    Blue,
    Green,
}

impl Color {
    fn print(&self){
        match self {
            Color::Blue => println!("Color of Box is Blue"),
            Color::Green => println!("Color of Box is Green"),
            Color::Red => println!("Color of Box is Red"),

        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self){
        println!("width is : {:?}", self.width);
        println!("height is : {:?}", self.height);
        println!("depth is : {:?}", self.depth);
    }
}
struct BoxCharacteristics {
    color : Color ,
    weight : f64 ,
    dimensions : Dimensions,
}

impl BoxCharacteristics {
    fn new(color: Color , weight: f64 , dimensions : Dimensions) -> Self {
        Self {
            color ,
            weight ,
            dimensions 
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight is : {:?}" , self.weight);
    }
}

fn main() {
    let small_box_dimensions = Dimensions { 
        width: 1.0 ,
        height: 2.0 ,
        depth: 0.5,
    };

    let small_box = BoxCharacteristics::new(Color::Blue, 2.1, small_box_dimensions);
    small_box.print();
}