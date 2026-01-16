// making computer items with string and vectors

struct PCItems {
    device: String ,
    model: String ,
}

fn pc_print(text: &str) {
    println!("Model is: {:?}",text);
}

fn main() {
    let pc = vec![
        PCItems {
            device: "CPU".to_owned(),
            model: "core i5 14400F".to_owned(),
        },
        PCItems {
            device: String::from("GPU"),
            model: String::from("RTX 4060"),
        },
        PCItems {
            device: "RAM".to_owned(),
            model: "Vengeance DDR5 32 GB".to_owned(),
        },
    ];
    
    for item in pc {
        println!("Device is : {:?}", item.device);
        pc_print(&item.model);
    }
}