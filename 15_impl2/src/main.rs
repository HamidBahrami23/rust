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



enum BoxColor {
    Blue,
    Red,
    Green,
}

fn which_boxcolor (col : BoxColor) {
    match col {
        BoxColor::Blue => "Blue" ,
        BoxColor::Green => "Green" ,
        BoxColor::Red => "Red" ,
    };
}

struct BoxCharac {
    dimensions: (f64, f64 , f64 ) ,
    weight: f64 ,
    color: () ,
}

impl BoxCharac {
    fn n_box () -> Self {
        Self{ 
            dimensions: (0.25, 0.2, 0.3) ,
            weight: 0.75 ,
            color: which_boxcolor(BoxColor::Blue),
        }
    } 

    fn print_charac(&self) {
        println!("dimentions: {:?}", self.dimensions);
        println!("weight: {:?}" , self.weight);
        println!("color: {:?}", self.color);
    }
}
fn main() {
    let box_new = BoxCharac::n_box();
    box_new.print_charac();
}
