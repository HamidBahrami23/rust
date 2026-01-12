
fn circle(radius: f64 ) -> (f64 , f64 , f64) {
    let pi = 3.14;
    let a = pi * radius * radius ;
    let s = 2. * pi * radius ;
    let d = 2. * radius ;
    (d , s , a)
}

fn main() {
    let rad = 3. ;
    let circl_info = circle(rad);
    println!("Circle info with Radius: {:?}", rad);
    println!("Diameter : {:?} ",circl_info.0 );
    println!("Circumference : {:?}",circl_info.1);
    println!("Area : {:?}", circl_info.2);
}

//this was 2026.01.12 Exercise