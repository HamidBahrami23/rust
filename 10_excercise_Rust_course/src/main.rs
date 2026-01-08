fn main() {
    println!("========= general Excercise =========");
    let two: i32 = add( 3 , -1);
    let mut three = add(two , 1);
    three = 6 + three ;
    println!("three variable equals to = {}" , three);
    println!("using function in print : {}" , add(-13 , 3));
    println!("using :? in println : {:?}" , two + 6);
    ifex(123);
    loopex();
    whileex();
}

fn whileex(){
    println!("========= while Excercise =========");
    let mut a = 0;
    while a != 5 {
        println!("a value equals to : {:?}" , a);
        a = a + 1;
    }
}

fn loopex(){
    println!("========= Loop Excercise =========");
    let mut a = 0;
    loop{
        if a == 5 {
            break;
        }
        println!("value of a  is : {:?}" , a);
        a = a +1 ;
    }
}

fn add(a: i32 , b: i32) ->i32 {
    a+b
}

fn ifex(a: i32){
    println!("========= if Excercise =========");
    println!("number input is : {:?}" , a);
    if a > 200 {
        println!("Huge number");
    } else if a > 99 {
        println!("Big Number");
    } else {
        println!("small number");
    }
}