use std::io;
fn main () {
    let mut input = String::new();

    println!("What is your name:");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Hi, {}", input.to_uppercase()q)
        },
        Err(e) => println!("Something wrong! ERR: {}", e)
    }
}