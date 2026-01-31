// this is easy practice just for calculating chicks seed usage from day 1 to week 24
// result is ~10Kg in 6 month
fn main() {
    // let mut lifetimeweek = vec![];
    let weekusage = vec![5 , 15 , 15 , 30 , 30 , 35 , 40 , 45 , 50 , 50 , 55 , 60 , 65 , 70 , 70 , 80 , 80 , 80 , 85 , 90 , 95 , 95 , 95 , 95];
    let mut totalusageinweek = vec![];
    for i in 0..=23 {
        totalusageinweek.push(7 * weekusage[i]);
    }
    println!("{:?}",totalusageinweek);
    
    let mut sumusage = 0;
    for i in 0..=23{
        sumusage = sumusage + totalusageinweek[i]
    }

    println!("sum of usage = {} grams" , sumusage);
}
