struct Tempreture {
    degree_c: f64,
}

impl Tempreture {
    fn show_temp(&self) {
        println!("{:?} degree C",self.degree_c);
    }
    
}

fn main() {
    let hot = Tempreture {
        degree_c: 42.2
    };
    hot.show_temp();

    let cold = Tempreture {
        degree_c : 5.1
    };

    cold.show_temp();
}
