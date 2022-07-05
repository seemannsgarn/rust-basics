fn main () {
    let age = 5;
    let name = "Patric";

    match age {
        0 => println!("Zero, cant be!"),
        1..=7 => println!("Where is your mom?"),
        18|21 => println!("You can drink here!"),
        _ => println!("This is default value")
    }

    match name {
        "SpongeBob" => println!("{} Squrepants", name),
        "Patric" => println!("{} Star", name),
        _ => println!("I don't know your name")
    }
}