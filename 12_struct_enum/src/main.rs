
enum FlavorDrink {
    Sparkling,
    Fruity,
    Sweet,
}

struct DrinkInfo {
    flavor: FlavorDrink,
    fluid: f64,
}

fn print_drink(drink: DrinkInfo){

    println!("========================");
    match drink.flavor {
        FlavorDrink::Fruity => println!("* Flavor: Fruity"),
        FlavorDrink::Sparkling => println!("* Flavor: Sparkling"),
        FlavorDrink::Sweet => println!("* Flavor: Sweet"),
    }

    println!("* fluid ounce: {:?}", drink.fluid);
}
fn main() {
    let drink1 = DrinkInfo {
        flavor: FlavorDrink::Sweet,
        fluid: 5.4,
    };

    let drink2 = DrinkInfo {
        flavor: FlavorDrink::Fruity,
        fluid: 5.7,
    }; 

    let drink3 = DrinkInfo {
        flavor: FlavorDrink::Sparkling,
        fluid: 6.3,
    };   

    print_drink(drink1);
    print_drink(drink2);
    print_drink(drink3);
}
