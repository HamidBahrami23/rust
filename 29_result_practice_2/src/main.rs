#[derive(Debug)]
enum MenuChoice {
    Start,
    Exit,
    MainMenu,
}

fn getchoice(input: &str) ->Result<MenuChoice , String> {
    match input {
        "MainMenu" => Ok(MenuChoice::MainMenu),
        "Exit" => Ok(MenuChoice::Exit),
        "Start" => Ok(MenuChoice::Start),
        _ => Err("Oops the Choice was not Found!".to_owned()),
    }
}

fn printchoice(input: &MenuChoice){
    println!("You Choosed {:?}", input);
}

fn pick_choice(input: &str) -> Result<() , String> { // () this means return nothing
    let choice = getchoice(input)?; // ? mark is the key in this line
    printchoice(&choice);
    Ok(()) // () this means return nothing
}

fn main() {
    let choice1 = pick_choice("Start");
    let choice2 = pick_choice("quit");
    let choice3 = pick_choice("Exit");

    println!("choice 1 value: {:?}" , choice1);
    println!("choice 2 value: {:?}" , choice2);
    println!("choice 3 value: {:?}" , choice3);

    println!("{:?}",getchoice("zz"));
    println!("{:?}",getchoice("MainMenu"));
}
